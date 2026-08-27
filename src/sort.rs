pub use crate::board::*;
use crate::uci::{HistoryTable, SearchStackEntry};
pub use partial_sort;

impl Board {
    pub fn find_best_nth(&mut self, mvs: &mut MoveList, n: usize) {
        let mut max_pos = n;
        let mut max_score = mvs.moves[n].1;
        
        for i in (n+1)..mvs.pos {
            if mvs.moves[i].1 > max_score {
                max_pos = i;
                max_score = mvs.moves[i].1;
            }
        }

        let imed = mvs.moves[n];
        mvs.moves[n] = mvs.moves[max_pos];
        mvs.moves[max_pos] = imed;
    }

    pub fn score_moves(&mut self, mvs: &mut MoveList, best_move: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) {
        for i in 0..mvs.pos {
            mvs.moves[i].1 = self.value(mvs.moves[i].0, best_move, ss, hist, ply);
        }
    }
    pub fn value(&self, mv: Move, bm: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) -> i32 {    
        // mvv-lva
        if mv == bm {
            return 999999999;
        }
        
        let posq = self.piece_on_sq_maybe(mv.to.trailing_zeros() as usize) as i32;

        if posq != 0 {
            return posq * 10000000 - mv.piece_type as i32
                + hist.probe_tactical(mv, posq as usize - 1);
        }

        if hist.get_killer(ply as i32).clone() == mv {
            return 9000000;
        }
        
        hist.probe(ss, mv, ply) as i32
    }

    pub fn score_moves_see(&mut self, mvs: &mut MoveList, best_move: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) {
        for i in 0..mvs.pos {
            mvs.moves[i].1 = self.value_see(mvs.moves[i].0, best_move, ss, hist, ply);
        }
    }
    pub fn value_see(&mut self, mv: Move, bm: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) -> i32 {    
        // mvv-lva
        if mv == bm {
            return 999999999;
        }
        
        let posq = self.piece_on_sq_maybe(mv.to.trailing_zeros() as usize) as i32;

        if posq != 0 {
            return posq * 10000000 - mv.piece_type as i32
                - if !self.see_threshold(mv, 0) { 1000000000 } else {0}
                + hist.probe_tactical(mv, posq as usize - 1);
        }

        if hist.get_killer(ply as i32).clone() == mv {
            return 9000000;
        }
        
        hist.probe(ss, mv, ply) as i32
    }
}