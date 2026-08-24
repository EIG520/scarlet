pub use crate::board::*;
use crate::uci::{HistoryTable, SearchStackEntry};
pub use partial_sort;

impl Board {
    pub fn sort(&mut self, mvs: &mut MoveList, best_move: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) {
        mvs.moves.sort_by_cached_key(|&a| 
            -self.value(a, best_move, ss, hist, ply)
        );
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

    pub fn sort_see(&mut self, mvs: &mut MoveList, best_move: Move, ss: &[SearchStackEntry], hist: &HistoryTable, ply: usize) {
        mvs.moves.sort_by_cached_key(|&a| 
            -self.value_see(a, best_move, ss, hist, ply)
        );
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