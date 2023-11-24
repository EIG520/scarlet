pub use crate::board::*;

impl Board {
    pub fn sort(&mut self, mvs: &mut MoveList) {
        mvs.moves.sort_by(
            |&a, &b| self.value(b).cmp(&self.value(a))
        )
    }
    pub fn value(&mut self, mv: Move) -> i32 {
        // Push all unfilled moves to the back
        if mv == Move::null() {return -99999;}
    
        // Get piece on square type, multiply by 100
        // then subtract piece type
        // so scuffed mvv-lva
        self.piece_on_sq(mv.to.trailing_zeros() as usize) as i32 * 100 - mv.piece_type as i32
    }
}