pub use crate::board::*;
pub use crate::utils::*;
pub use std::time::Instant;

impl Board {
    pub fn perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
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

        let mut moves = MoveList::new();
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
}

impl<'a> Searcher<'a> {
    pub fn new(board: &'a mut Board) -> Self {
        Self {
            board: board,
            root_best: Move::null(),
            search_best: Move::null(),
            root_best_eval: -99999,
            search_best_eval: -99999,
            search_ms: 0,
            nodes: 0,
        }
    }

    pub fn search(&mut self, depth: i32, mut alpha: i32, beta: i32, ply: u32, timer:Instant) -> i32 {
        self.nodes += 1;
        // Detect repetition
        // Repetition goes by 2 in this instead of 3
        // So don't do this in root
        let root: bool = ply == 0;
        let qsearch: bool = depth <= 0;

        if self.board.is_repetition() && !root {return 0;}

        // Get all legal moves
        let mut mvs: MoveList = MoveList::new();

        // Qsearch
        if qsearch {
            self.board.gen_legal_moves(&mut mvs, true);

            let stand_pat = self.board.evaluate();

            if stand_pat >= beta {
                return beta;
            }
            if alpha < stand_pat {
                alpha = stand_pat;
            }
        } else {
            self.board.gen_legal_moves(&mut mvs, false);
        }

        self.board.sort(&mut mvs);

        // Main Search
        let mut best = -99999999;

        for i in 0..mvs.pos {

            if timer.elapsed().as_millis() >= self.search_ms && depth > 2 && self.search_ms != 0 {
                return 99999999;
            }

            let mv = mvs.moves[i as usize];

            self.board.make_move(&mv);

            let eval = -self.search(depth-1,  -beta, -alpha, ply + 1, timer);
            
            self.board.undo();

            if eval > best {
                best = eval;

                if root {
                    self.search_best = mv;
                    self.search_best_eval = eval;
                }

                // Alpha beta pruning (~gazillion elo)
                if eval > alpha {alpha = eval;}
                if alpha >= beta {break;}
            }
        }

        if root && (self.search_ms < timer.elapsed().as_millis() || self.search_best_eval > self.root_best_eval) {
            self.root_best = self.search_best;
            self.root_best_eval = self.search_best_eval;
        }

        if mvs.pos == 0 {
            if qsearch {
                return self.board.evaluate();
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

        println!("info depth {} nodes {} ", depth, self.nodes);
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
                println!("info depth {} nodes {} nps {} score cp {} pv {} time {}", depth, self.nodes, 1000 * self.nodes / timer.elapsed().as_millis(), self.root_best_eval, move_to_chess(self.root_best), timer.elapsed().as_millis());
            }
        }
        println!("bestmove {}", move_to_chess(self.root_best));
        self.root_best
    }
}