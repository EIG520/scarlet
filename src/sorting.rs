pub use crate::board;

pub fn sort(mvs: &mut [(u64, u64, usize, usize); 300]) {
    mvs.sort_by(
        |&a, &b| value(b).cmp(&value(a))
    )
}

pub fn value(mv: (u64, u64, usize, usize)) -> i32 {
    // Push all unfilled moves to the back
    if mv == (0,0,0,0) {return -99999;}

    // Get piece on square type, multiply by 100
    // then subtract piece type
    // so scuffed mvv-lva
    board::piece_on_sq(mv.1.trailing_zeros() as usize) as i32 * 100 - mv.2 as i32
}