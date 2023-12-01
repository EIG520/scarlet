pub use crate::board::*;

impl Board {
    pub fn sort(&mut self, mvs: &mut MoveList) {
        mvs.moves.sort_unstable_by_key(|&a| -self.value(a));
    }
    pub fn value(&self, mv: Move) -> i32 {    
        // mvv-lva
        let posq = self.piece_on_sq(mv.to.trailing_zeros() as usize) as i32;

        if posq != 0 {
            return posq * 100 - mv.piece_type as i32;
        }
        0
    }
}