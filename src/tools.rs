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
    if depth == 0 {
        return 1;
    }

    let mut counter: i32 = 0;
    let mut i: usize = 0;

    let mut mvs = vec![] ;

    let cnt = moves::pslegalmoves(&mut mvs);
    // // Can't do this with pseudo legal move gen :(
    // if (depth == 1) & !root {
    //     return mvs.len() as i32;
    // }

    while i < cnt as usize{
        board::movebb(mvs[i].0, mvs[i].1, mvs[i].2);
        
        board::change_turn();
        if !board::in_check() {
            board::change_turn();
            let sub: i32 = perft_depth(depth-1, false);

            if root {
                println!("{}: {}", board::move_to_chess(mvs[i]), sub);
            }

            counter += sub;
        } else {
            board::change_turn();
        }
        board::undo();

        i += 1;
    }

    return counter;
}

