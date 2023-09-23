pub use scarlet::board;
pub use scarlet::moves;
pub use scarlet::tools;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    // let mut mv: (u64, u64, usize) = board::chess_to_move(String::from("f2f4"), 0);

    // board::movebb(mv.0, mv.1, mv.2);

    // mv = board::chess_to_move(String::from("g7g5"), 1);

    // board::movebb(mv.0, mv.1, mv.2);
    
    // mv = board::chess_to_move(String::from("e1f2"), 10);

    // board::movebb(mv.0, mv.1, mv.2);

    // mv = board::chess_to_move(String::from("g5f4"), 1);

    // board::movebb(mv.0, mv.1, mv.2);
    
    println!("{}", tools::perft_depth(6, true));

    println!("Time: {:?}", start.elapsed())

    /*
    let mut mv: (u64, u64, usize) = board::chess_to_move(String::from("e2e4"), 0);

    board::movebb(mv.0, mv.1, mv.2);
    */
}

