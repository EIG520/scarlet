pub use crate::board;
pub use crate::moves;

static mut CNUM: usize = 48039487;

pub fn random_game() -> i32 {
    let mut mvs: [(u64,u64,usize,usize); 300] = [(0,0,0,0); 300];
    let mut len: i32 = 0;

    let mut state = board::state();

    while state == 0 {
        let mvcnt = moves::legalmoves(&mut mvs);

        let mv = mvs[psrandom(mvcnt as usize)];

        board::movebb(mv.0, mv.1, mv.2, mv.3);
        len += 1;

        state = board::state();
    }

    // This will be used to find the winner
    let cside = board::color() as i32;

    for _ in 0..len {
        board::undo();
    }
    //board::print_bb(board::get_bitboard(12) | board::get_bitboard(13));


    if state == 2 {
        // The side to move is the loser so
        // 0 * 2 - 1 = -1 (-1 for black winning)
        // 1 * 2 - 1 = 1 (1 for white winning)
        return cside * 2 - 1;
    }
    0
}

pub fn rscore(ss: i32) -> i32 {
    let color = board::color() as i32 * 2 - 1;
    let mut cscore = 0;
    for _ in 0..ss {
        let i = random_game();
        cscore += i;
        //println!("{}", i);

    }
    cscore * color
}

pub fn bestmove() -> (u64, u64, usize, usize) {
    let mut mvs: [(u64, u64, usize, usize); 300] = [(0,0,0,0); 300];
    let cnt = moves::legalmoves(&mut mvs);

    let mut best_mv: (u64, u64, usize, usize) = (0,0,0,0);
    let mut best_sc = -9999999;
    for i in 0..cnt {
        let mv = mvs[i as usize];

        board::movebb(mv.0, mv.1, mv.2, mv.3);

        let sc = rscore(1000);

        //println!("{}: {}",board::move_to_chess(mv), sc);

        if sc > best_sc {
            best_mv = mv;
            best_sc = sc;
        }

        board::undo();
    }
    
    println!("info score cp {}", best_sc as f32 / 10.0);

    best_mv
}


pub fn psrandom(max: usize) -> usize {
    unsafe {
        CNUM = CNUM.wrapping_mul(490258253);
        CNUM = CNUM.wrapping_add(908470237);

        return CNUM % max;
    }
}
