pub use crate::moves;
pub use crate::board;

// Used for making lookup tables
pub fn gen_movelist() {
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
pub fn perft_lim(lim: i32) {
    for i in 0..lim {
        println!("{}", perft_depth(i, false));
    }
}

// fixed perft
pub fn perft_depth(depth: i32, root: bool) -> i32{
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

