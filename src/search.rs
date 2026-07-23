pub use crate::board::*;
pub use crate::transposition_table::*;
pub use crate::uci::*;
use std::time::Instant;

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
    transposition_table: &'a mut TranspositionTable,
    history_table: HistoryTable,
    options: StoredOptions,
}

impl<'a> Searcher<'a> {
    pub fn new(board: &'a mut Board, transposition_table: &'a mut TranspositionTable, options: StoredOptions) -> Self {
        Self {
            board,
            root_best: Move::null(),
            search_best: Move::null(),
            root_best_eval: -30000,
            search_best_eval: -30000,
            search_ms: 0,
            nodes: 0,
            transposition_table,
            history_table: HistoryTable::default(),
            options
        }
    }

    pub fn search(&mut self, mut depth: i32, mut alpha: i32, beta: i32, ply: u32, timer:Instant) -> i32 {
        if depth <= 0 { return self.qsearch(alpha, beta, ply) }
        
        self.nodes += 1;

        let root: bool = ply == 0;
        let incheck = self.board.in_check();
        let pv = alpha != beta - 1;
        let reduce = !pv && !incheck;

        if self.board.is_repetition() && !root {return 0;}

        let mut tt_entry: Option<TranspositionInfo> = None;

        if self.options.use_tt {
            tt_entry = self.transposition_table
                .probe(self.board);
        }

        let mut stat = self.board.eval();

        if let Some(entry) = tt_entry {
            let score = entry.score;

            if !pv && entry.depth as i32 >= depth && match entry.fail {
                Fail::NoFail => true,
                Fail::FailHigh => score >= beta,
                Fail::FailLow => score <= alpha,
                Fail::None => false, // not possible
            } {
                return score;
            }

            // Use TT eval instead of static evaluation
            stat = score;
        } else if depth > 4 {
            depth -= 1;
        }


        // Pruning
        if !root && reduce {
            // rfp
            if stat - 85 * depth >= beta { return stat; }

            // null move pruning
            if depth > 2 && stat >= beta {
                self.board.make_null_move();

                let eval = -self.search((depth * 100 + beta - stat) / 200 - 1, -beta, 1-beta, ply + 1, timer);
            
                self.board.unmake_null_move();

                if eval >= beta { return eval; }
            }
        }

        let mut mvs = MoveList::default();
        self.board.gen_legal_moves(&mut mvs, false);

        if let Some(entry) = tt_entry {
            self.board.sort_see(&mut mvs, entry.best_move, &self.history_table, ply);
        } else {
            self.board.sort_see(&mut mvs, Move::null(), &self.history_table, ply);
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

            let is_capture = mv.to & (self.board.get_bitboard(PieceType::WhitePieces) | self.board.get_bitboard(PieceType::BlackPieces)) != 0;
            let is_qpromo = mv.flag == Flag::QueenPromotion;

            // moveloop pruning (thank you amber21!!!)
            if !root && !incheck && best >= -20000 {

                // late move pruning
                if !pv && !is_capture && !is_qpromo && i as i32 > 5 + depth * depth && alpha.abs() < 2000 && beta < 20000 {
                    break;
                }

                // futility pruning
                if !is_capture && !is_qpromo && stat + 256 + 128 * depth < alpha && alpha.abs() < 2000 && depth <= 6 {
                    break;
                }


            }

            self.board.make_move(&mv);

            let mut ext = 0;

            if self.board.in_check() {
                ext += 1;
            }

            let newdepth = depth - 1 + ext;

            let mut eval = 67;

            let ireq = if pv { 999999 } else { 2 };

            if i > ireq && depth >= 2 {
                let reduction = (
                    1.0 + (depth as f32).ln() * (i as f32).ln() / 2.0
                    - self.history_table.probe(mv) as f32 / 200.0
                ).floor() as i32;

                let reduced = (newdepth - reduction).clamp(0, depth - 1);

                eval = -self.search(reduced, -alpha-1, -alpha, ply + 1, timer);

                if eval > alpha && reduced < newdepth {
                    eval = -self.search(newdepth, -alpha-1, -alpha, ply + 1, timer);
                }
            } else if !pv || i > 0 {
                eval = -self.search(newdepth, -alpha-1, -alpha, ply + 1, timer);
            }

            if pv && (i == 0 || eval > alpha) {
                eval = -self.search(newdepth, -beta, -alpha, ply + 1, timer);
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
                if eval > alpha {
                    alpha = eval;
                    mvtype = Fail::NoFail;
                }
                if alpha >= beta {
                    mvtype = Fail::FailHigh;
                    
                    if !is_capture {
                        self.history_table.apply_delta(mv, depth * depth);

                        for j in 0..i {
                            let mv2 = mvs.moves[j];
                            let is_capture_2 = mv2.to & (self.board.get_bitboard(PieceType::WhitePieces) | self.board.get_bitboard(PieceType::BlackPieces)) != 0;

                            if !is_capture_2 {
                                self.history_table.apply_delta(mv2, - depth * depth);
                            }
                        }
                    }

                    if !is_capture {
                        self.history_table.add_killer(mv, ply as i32);
                    }

                    break;
                }
            }
        }

        if mvs.pos == 0 {
            // in check & no moves = mate
            if incheck {
                return ply as i32 - 30000;
            }
            return 0;
        }

        if self.options.use_tt  {
            self.transposition_table.add(
                self.board, depth as i8, best, 
                    if mvtype != Fail::FailLow || tt_entry.is_none() { best_move }
                            else { tt_entry.unwrap().best_move }
                , mvtype
            );
        }

        if root && (timer.elapsed().as_millis() < self.search_ms || self.search_best_eval > self.root_best_eval) {
            self.root_best = self.search_best;
            self.root_best_eval = self.search_best_eval;
        }


        best
    }

    fn qsearch(&mut self, mut alpha: i32, beta: i32, ply: u32) -> i32 {
        self.nodes += 1;

        // let incheck = self.board.in_check();
        // let pv = alpha != beta - 1;

        if self.board.is_repetition() {return 0;}
        
        let mut tt_entry: Option<TranspositionInfo> = None;

        if self.options.use_tt {
            tt_entry = self.transposition_table
                .probe(self.board);
        }

        if let Some(entry) = tt_entry {
            let score = entry.score;

            if match entry.fail {
                Fail::NoFail => true,
                Fail::FailHigh => score >= beta,
                Fail::FailLow => score <= alpha,
                Fail::None => false, // not possible
            } {
                return score;
            }
        }

        // Stand pat check
        let stat = self.board.eval();
        if stat >= beta {
            return beta;
        }
        if alpha < stat {
            alpha = stat;
        }

        // Move generation
        let mut mvs = MoveList::default();
        self.board.gen_legal_moves(&mut mvs, true);
        if let Some(entry) = tt_entry {
            self.board.sort(&mut mvs, entry.best_move, &self.history_table, ply);
        } else {
            self.board.sort(&mut mvs, Move::null(), &self.history_table, ply);
        }

        // Main search
        let mut best = -30000;

        for i in 0..mvs.pos {
            let mv = mvs.moves[i];
            
            self.board.make_move(&mv);

            let eval = -self.qsearch(-beta, -alpha, ply + 1);

            self.board.undo();

            if eval > best {
                best = eval;

                if eval > alpha {
                    alpha = eval;
                }

                if alpha >= beta {
                    break;
                }
            }
        }

        if mvs.pos == 0 {
            return stat;
        }

        best
    }

    pub fn search_to_depth(&mut self, depth: i32) -> Move {
        self.history_table = HistoryTable::default();
        self.search_ms = 0;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;

        let timer = Instant::now();

        self.search(depth as i32, -30000, 30000, 0, timer);

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
            .probe(self.board);

        if let Some(info) = entry {
            if info.fail == Fail::NoFail {
                return Some(info.best_move)
            }
        }
        None
    }

    pub fn search_for_ms(&mut self, ms: u128) -> Move {
        self.history_table = HistoryTable::default();
        self.search_ms = ms;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;
        let timer = Instant::now();

        let mut depth = 0;

        // Go deeper and deeper until either mate is found or time is up
        while timer.elapsed().as_millis() < self.search_ms / 2 && self.search_best_eval < 20000 {
            if depth == 100 {break}

            depth += 1;
            let peval = self.root_best_eval;

            self.search(depth, peval - 20, peval + 20, 0, timer);

            if self.root_best_eval <= peval - 20 || self.root_best_eval >= peval + 20 {
                self.search(depth, -30000, 30000, 0, timer);
            }

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