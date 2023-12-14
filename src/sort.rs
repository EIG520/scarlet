pub use crate::board::*;
pub use partial_sort;

impl Board {
    pub fn sort(&mut self, mvs: &mut MoveList, best_move: Move) {
        mvs.moves.sort_unstable_by_key(|&a| 
            -self.value(a, best_move)
        );
    }
    pub fn value(&self, mv: Move, bm: Move) -> i32 {    
        // mvv-lva
        if mv == bm {
            return 9999999;
        }
        
        let posq = self.piece_on_sq(mv.to.trailing_zeros() as usize) as i32;

        if posq != 0 {
            return posq * 100 - mv.piece_type as i32;
        }
        0
    }
}