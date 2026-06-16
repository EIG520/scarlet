pub use crate::board::*;
pub use crate::transposition_table::*;
pub use crate::uci::*;
use std::cmp::min;
use std::time::Instant;
use std::sync::RwLock;

impl Board {
    pub fn perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::default();
        self.gen_legal_moves(&mut moves, false);

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i];
            
            self.make_move(&mv);

            let subcount = self.sub_perft(depth - 1);

            self.undo();

            count += subcount;
            
            println!("{}: {}", move_to_chess(mv), subcount);
        }
        println!("nodes: {}", count);
        count
    }
    pub fn sub_perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::default();
        self.gen_legal_moves(&mut moves, false);

        if depth == 1 {
            return moves.pos as u64;
        }

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i];
            
            self.make_move(&mv);
            
            count += self.sub_perft(depth - 1);

            self.undo();
        }
        count
    }
}

pub struct Searcher<'a> {
    board: &'a mut Board,
    root_best: Move,
    search_best: Move,
    root_best_eval: i32,
    search_best_eval: i32,
    search_ms: u128,
    nodes: u128,
    transposition_table: &'a RwLock<TranspositionTable>,
    options: StoredOptions,
}

impl<'a> Searcher<'a> {
    pub fn new(board: &'a mut Board, transposition_table: &'a RwLock<TranspositionTable>, options: StoredOptions) -> Self {
        Self {
            board,
            root_best: Move::null(),
            search_best: Move::null(),
            root_best_eval: -30000,
            search_best_eval: -30000,
            search_ms: 0,
            nodes: 0,
            transposition_table,
            options
        }
    }

    pub fn search(&mut self, depth: i32, mut alpha: i32, beta: i32, ply: u32, donull: bool, timer:Instant) -> i32 {
        self.nodes += 1;

        let root: bool = ply == 0;
        self.board.gen_hit_squares(); // maybe remove this line later
        let incheck = self.board.attacked() & self.board.get_bitboard(PieceType::WhiteKing.shiftedby(self.board.color())) > 0;
        let pv = alpha != beta - 1;
        let qsearch: bool = depth <= 0;
        let reduce = !pv && !incheck;

        if self.board.is_repetition() && !root {return 0;}

        let mut tt_entry: Option<TranspositionInfo> = None;

        if !qsearch && self.options.use_tt {
            tt_entry = self.transposition_table
                .read()
                .expect("failed to read rwlock")
                .probe(self.board);
        }

        if tt_entry.is_some() && !root {
            let entry = tt_entry.unwrap();

            if entry.depth as i32 >= depth && (
                entry.fail == Fail::NoFail
                || entry.fail == Fail::FailHigh && entry.score >= beta
                || entry.fail == Fail::FailLow && entry.score <= alpha) 
            {return entry.score}
        }

        let stat = self.board.eval();

        // Qsearch
        if qsearch {
            let stand_pat = self.board.eval();

            if stand_pat >= beta {
                return beta;
            }
            if alpha < stand_pat {
                alpha = stand_pat;
            }
        } else if !root && reduce {
            // rfp
            if stat - 85 * depth >= beta { return stat; }

            // null move pruning
            if donull && depth > 2 {
                self.board.make_null_move();

                let eval = -self.search(depth - 3, -beta, 1-beta, ply + 1, false, timer);
            
                self.board.unmake_null_move();

                if eval >= beta { return eval; }
            }
        }

        let mut mvs = MoveList::default();
        self.board.gen_legal_moves(&mut mvs, qsearch);

        if let Some(entry) = tt_entry {
            self.board.sort(&mut mvs, entry.best_move);
        } else {
            self.board.sort(&mut mvs, Move::null());
        }

        // Main Search
        let mut best = -30000;
        let mut best_move = Move::null();
        let mut mvtype = Fail::FailLow;

        for i in 0..mvs.pos {

            if self.nodes % 2048 == 0 && self.search_ms != 0 && timer.elapsed().as_millis() >= self.search_ms {
                return 30000;
            }

            let mv = mvs.moves[i];


            self.board.make_move(&mv);

            let mut eval;
            if i > 0 {
                eval = -self.search(depth-1,  -alpha - 1, -alpha, ply + 1, donull, timer);

                if eval > alpha && eval < beta {
                    eval = -self.search(depth-1,  -beta, -alpha, ply + 1, donull, timer);
                }
            } else {
                eval = -self.search(depth-1,  -beta, -alpha, ply + 1, donull, timer);
            }


            
            self.board.undo();

            // if root {
            //     println!("move {} eval {}", move_to_chess(mv), eval);
            // }

            if eval > best {
                best = eval;
                best_move = mv;

                if root {
                    self.search_best = mv;
                    self.search_best_eval = eval;
                }

                // Alpha beta pruning
                if eval > alpha {alpha = eval;mvtype = Fail::NoFail}
                if alpha >= beta {mvtype = Fail::FailHigh;break;}
            }
        }

        if mvs.pos == 0 {
            if qsearch {
                return self.board.eval();
            }
            // in check & no moves = mate
            if self.board.checkmask() != u64::MAX {
                return ply as i32 - 30000;
            }
            return 0;
        }

        if !qsearch && self.options.use_tt  {
            self.transposition_table.write().expect("failed to lock transposition table").add(
                self.board, depth as i8, best, best_move, mvtype
            );
        }

        if root && (timer.elapsed().as_millis() < self.search_ms || self.search_best_eval > self.root_best_eval) {
            self.root_best = self.search_best;
            self.root_best_eval = self.search_best_eval;
        }


        best
    }

    pub fn search_to_depth(&mut self, depth: i32) -> Move {
        self.search_ms = 0;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;

        let timer = Instant::now();

        self.search(depth as i32, -30000, 30000, 0, true, timer);

        if timer.elapsed().as_millis() > 0 {
            print!("info depth {} nodes {} nps {} score cp {} time {}", depth, self.nodes, 1000 * self.nodes / timer.elapsed().as_millis(), self.root_best_eval, timer.elapsed().as_millis());
        } else {
            print!("info depth {} nodes {} score cp {} time {}", depth, self.nodes, self.root_best_eval, timer.elapsed().as_millis());
        }

        print!(" pv");

        let mut tm = self.top_move();
        
        let mut mvs = 0;
        while tm.is_some() {
            print!(" {}", move_to_chess(tm.unwrap()));

            self.board.make_move(&tm.unwrap());

            tm = self.top_move();

            mvs += 1;
        }

        while mvs > 0 {
            self.board.undo();
            mvs -= 1;
        }

        println!();
        println!("bestmove {}", move_to_chess(self.root_best));

        self.root_best
    }

    pub fn top_move(&self) -> Option<Move> {
        let entry = self.transposition_table
            .read()
            .expect("")
            .probe(self.board);

        if let Some(info) = entry {
            if info.fail == Fail::NoFail {
                return Some(info.best_move)
            }
        }
        None
    }

    pub fn search_for_ms(&mut self, ms: u128) -> Move {
        self.search_ms = ms;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;
        let timer = Instant::now();

        let mut depth = 0;

        // Go deeper and deeper until either mate is found or time is up
        while timer.elapsed().as_millis() < self.search_ms && self.search_best_eval < 20000 {
            if depth == 100 {break}

            depth += 1;
            self.search(depth, -30000, 30000, 0, true, timer);

            if timer.elapsed().as_millis() > 0 {
                print!("info depth {} nodes {} nps {} score cp {} time {}", depth, self.nodes, 1000 * self.nodes / timer.elapsed().as_millis(), self.root_best_eval, timer.elapsed().as_millis());
            } else {
                print!("info depth {} nodes {} score cp {} time {}", depth, self.nodes, self.root_best_eval, timer.elapsed().as_millis());
            }

            let mut tm = self.top_move();
            
            let mut mvs = 0;
            while tm.is_some() {
                if self.board.is_repetition() {break;}

                if mvs == 0 {print!(" pv");}

                print!(" {}", move_to_chess(tm.unwrap()));
                self.board.make_move(&tm.unwrap());
                tm = self.top_move();
                mvs += 1;
            }
            
            while mvs > 0 {
                self.board.undo();
                mvs -= 1;
            }

            println!();
        }
        println!("bestmove {}", move_to_chess(self.root_best));
        self.root_best
    }
}