pub use scarlet::board;
pub use scarlet::moves;
pub use scarlet::tools;

use std::time::Instant;

fn main() {
    board::load_from_fen(String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));

    let start = Instant::now();

    // let mut mv: (u64, u64, usize, usize) = board::chess_to_move("b4b1".to_string(), 6, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);


    // mv = board::chess_to_move("h4g3".to_string(), 11, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("a5b4".to_string(), 10, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("g3f2".to_string(), 11, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("b4c3".to_string(), 10, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("f2e2".to_string(), 11, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);
    
    //println!("{}", board::get_bitboard(14));

    println!("{}", tools::perft_nobatch(6,true));
    

    println!("Time: {:?}", start.elapsed());
}

