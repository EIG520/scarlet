pub use scarlet::board;
pub use scarlet::moves;
pub use scarlet::tools;
pub use scarlet::uci;

//use std::time::Instant;

fn main() {
    //let start = Instant::now();

    // let mut mv: (u64, u64, usize, usize) = board::chess_to_move("a2a4".to_string(), 0, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("h7h5".to_string(), 1, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("a4a5".to_string(), 0, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("b7b5".to_string(), 1, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("h5g6".to_string(), 0, 1);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);
    // mv = board::chess_to_move("b4c3".to_string(), 10, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);

    // mv = board::chess_to_move("f2e2".to_string(), 11, 0);
    // board::movebb(mv.0, mv.1, mv.2, mv.3);
    
    // for _i in 0..1000000 {
    //     moves::legalmoves(&mut [(0,0,0,0);300]);
    // }
    uci::uci();
    
    //println!("Time: {:?}", start.elapsed());
}
