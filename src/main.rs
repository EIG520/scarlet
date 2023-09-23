pub use scarlet::board;
pub use scarlet::moves;
use std::env;
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
    
    println!("{}", perft_depth(6, true));

    println!("Time: {:?}", start.elapsed())

    /*
    let mut mv: (u64, u64, usize) = board::chess_to_move(String::from("e2e4"), 0);

    board::movebb(mv.0, mv.1, mv.2);
    */
}

// Used for making lookup tables
fn gen_movelist() {
    let mut bb: u64;
    for i in 0..64 {
        bb = 0;
        // Replace with piece you need
        for mv in moves::black_pawn_doub(i) {
            bb |= 1 << mv.to;
        }
        print!("{:#018x}", bb);
        print!(", ");
    }
}

// perft until a given limit
fn perft_lim(lim: i32) {
    for i in 0..lim {
        println!("{}", perft_depth(i, false));
    }
}

// fixed perft
fn perft_depth(depth: i32, root: bool) -> i32{
    board::change_turn();
    if board::in_check() {
        board::change_turn();
        return 0;
    }
    board::change_turn();
    if depth == 0 {
        return 1;
    }

    let mut counter: i32 = 0;

    let mvs = moves::pslegalmoves();

    // // Can't do this with pseudo legal move gen :(
    // if (depth == 1) & !root {
    //     return mvs.len() as i32;
    // }

    for mv in mvs {
        board::movebb(mv.0, mv.1, mv.2);
        
        let sub: i32 = perft_depth(depth-1, false);

        if root {
           println!("{}: {}", board::move_to_chess(mv), sub);
        }

        counter += sub;

        board::undo();
    }

    return counter;
}

