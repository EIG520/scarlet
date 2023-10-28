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
        println!("{}", perft(i, false));
    }
}

// perft without batch (also pseudolegal)
pub fn perft_nobatch(depth: i32, root: bool) -> u64{
    if depth == 0 {
        return 1;
    }

    let mut counter: u64 = 0;
    let mut i: usize = 0;

    let mut mvs = vec![] ;

    let cnt = moves::legalmoves(&mut mvs);
    // // Can't do this with pseudo legal move gen :(
    // if (depth == 1) & !root {
    //     return mvs.len() as u64;
    // }

    //println!("{:?}", mvs);

    while i < cnt as usize{
        board::movebb(mvs[i].0, mvs[i].1, mvs[i].2, mvs[i].3);

        board::change_turn();
        if !board::in_check() {
            board::change_turn();
            let sub: u64 = perft_nobatch(depth-1, false);

            if root {
                println!("{}: {}", board::move_to_chess(mvs[i]), sub);
            }

            counter += sub;
        } else {
            //println!("{}", board::move_to_chess(mvs[i]));
            board::change_turn();
        }

        board::undo();

        i += 1;
    }

    return counter;
}

// perft
pub fn perft(depth: i32, root: bool) -> u64{
    if depth == 0 {
        return 1;
    }

    let mut counter: u64 = 0;
    let mut i: usize = 0;

    let mut mvs = [(0,0,0,0);300];
    let cnt = moves::legalmoves(&mut mvs);

    if (depth == 1) & !root {
        return cnt as u64;
    }

    while i < cnt as usize{
        board::movebb(mvs[i].0, mvs[i].1, mvs[i].2, mvs[i].3);

        let sub: u64 = perft(depth-1, false);

        if root {
            println!("{}: {}", board::move_to_chess(mvs[i]), sub);
        }

        counter += sub;

        board::undo();

        i += 1;
    }

    return counter;
}

// pub fn sort_moves(arr: &mut [(u64,u64,usize,usize)]) {

//     let mut scores = *arr.clone();

//     for x in arr {
//         // If all zero, break out of loop
//         if x.0 | x.1 | x.2 as u64 | x.3 as u64 == 0 {break;}

//     }
// }