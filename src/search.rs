pub use crate::board::*;
pub use crate::utils::*;
pub use crate::transposition_table::*;
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
}

impl<'a> Searcher<'a> {
    pub fn new(board: &'a mut Board, transposition_table: &'a RwLock<TranspositionTable>) -> Self {
        Self {
            board,
            root_best: Move::null(),
            search_best: Move::null(),
            root_best_eval: -99999,
            search_best_eval: -99999,
            search_ms: 0,
            nodes: 0,
            transposition_table,
        }
    }

    pub fn search(&mut self, depth: i32, mut alpha: i32, beta: i32, ply: u32, timer:Instant) -> i32 {
        self.nodes += 1;

        let root: bool = ply == 0;
        let _pv = alpha != beta - 1;
        let qsearch: bool = depth <= 0;

        if self.board.is_repetition() && !root {return 0;}



        // bring legal moves list into scope
        let mut mvs: MoveList;

        // Qsearch
        if qsearch {
            let stand_pat = self.board.eval();

            if stand_pat >= beta {
                return beta;
            }
            if alpha < stand_pat {
                alpha = stand_pat;
            }
            // Only calc (and allocate space for) moves
            // if we really know we need them
            mvs = MoveList::default();
            self.board.gen_legal_moves(&mut mvs, true);
            self.board.sort(&mut mvs, Move::null());
        } else {
            mvs = MoveList::default();
            self.board.gen_legal_moves(&mut mvs, false);

            let tt_entry: Option<TranspositionInfo>;

            tt_entry = self.transposition_table
                .read()
                .expect("failed to read rwlock")
                .probe(self.board);
    
            if tt_entry.is_some() {
                let entry = tt_entry.unwrap();
    
                if entry.depth as i32 >= depth && (
                    entry.fail == (false, false)
                    || entry.fail == (true, false) && entry.score <= alpha
                    || entry.fail == (false, true) && entry.score >= beta) 
                {return entry.score}

                self.board.sort(&mut mvs, tt_entry.unwrap().best_move);
            } else {
                self.board.sort(&mut mvs, Move::null());
            }
        }

        // Main Search
        let mut best = -99999999;
        let mut best_move = Move::null();
        let mut mvtype = (true, false);

        for i in 0..mvs.pos {

            if self.nodes % 2048 == 0 && self.search_ms != 0 && timer.elapsed().as_millis() >= self.search_ms {
                return 99999999;
            }

            let mv = mvs.moves[i];

            self.board.make_move(&mv);

            let mut eval = -self.search(depth-1,  -alpha - 1, -alpha, ply + 1, timer);

            if eval > alpha && eval < beta {
                eval = -self.search(depth-1,  -beta, -alpha, ply + 1, timer);
            }
            
            self.board.undo();

            if eval > best {
                best = eval;
                best_move = mv;

                if root {
                    self.search_best = mv;
                    self.search_best_eval = eval;
                }

                // Alpha beta pruning
                if eval > alpha {alpha = eval;mvtype = (false, false)}
                if alpha >= beta {mvtype = (false, true);break;}
            }
        }

        self.transposition_table.write().expect("failed to lock transposition table").add(
            self.board, depth as i16, best, best_move, mvtype
        );

        if root && (timer.elapsed().as_millis() < self.search_ms || self.search_best_eval > self.root_best_eval) {
            self.root_best = self.search_best;
            self.root_best_eval = self.search_best_eval;
        }

        if mvs.pos == 0 {
            if qsearch {
                return self.board.eval();
            }
            // in check & no moves = mate
            if self.board.checkmask() != u64::MAX {
                return ply as i32 - 999999;
            }
            return 0;
        }
        best
    }

    pub fn search_to_depth(&mut self, depth: i32) -> Move {
        self.search_ms = 0;
        self.root_best_eval = -999999;
        self.search_best_eval = -999999;
        self.nodes = 0;

        let timer = Instant::now();

        self.search(depth, -999999, 999999, 0, timer);

        if timer.elapsed().as_millis() > 0 {
            println!("info depth {} nodes {} nps {} score cp {} time {} pv {}", depth, self.nodes, 1000 * self.nodes / timer.elapsed().as_millis(), self.root_best_eval, timer.elapsed().as_millis(), move_to_chess(self.root_best));
        } else {
            println!("info depth {} nodes {} score cp {} time {} pv {}", depth, self.nodes, self.root_best_eval, timer.elapsed().as_millis(), move_to_chess(self.root_best));
        }
        println!("bestmove {}", move_to_chess(self.root_best));

        self.root_best
    }

    pub fn search_for_ms(&mut self, ms: u128) -> Move {
        self.search_ms = ms;
        self.root_best_eval = -999999;
        self.search_best_eval = -999999;
        self.nodes = 0;
        let timer = Instant::now();

        let mut depth = 0;
        while timer.elapsed().as_millis() < self.search_ms {
            depth += 1;
            self.search(depth, -999999, 999999, 0, timer);
            if timer.elapsed().as_millis() > 0 {
                println!("info depth {} nodes {} nps {} score cp {} time {} pv {}", depth, self.nodes, 1000 * self.nodes / timer.elapsed().as_millis(), self.root_best_eval, timer.elapsed().as_millis(), move_to_chess(self.root_best));
            } else {
                println!("info depth {} nodes {} score cp {} time {} pv {}", depth, self.nodes, self.root_best_eval, timer.elapsed().as_millis(), move_to_chess(self.root_best));
            }
        }
        println!("bestmove {}", move_to_chess(self.root_best));
        self.root_best
    }
}