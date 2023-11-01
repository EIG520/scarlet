pub use crate::board;
pub use crate::moves;
pub use crate::eval;
pub use crate::sorting;
use std::time::Instant;

static mut SEARCH_MS: u128 = 0;

static mut ROOT_BEST_MOVE: (u64, u64, usize, usize) = (0,0,0,0);
static mut SEARCH_BEST_MOVE: (u64, u64, usize, usize) = (0,0,0,0);
static mut ROOT_EVAL: i32 = 0;
static mut SEARCH_EVAL: i32 = 0;

pub fn search(depth: i32, root: bool, mut alpha: i32, beta: i32, ply: i32, timer: Instant) -> i32{
    // Detect repetition
    // Repetition goes by 2 in this instead of 3
    // So don't do this in root
    if board::is_repetition() && !root {return 0;}

    let qsearch: bool = depth <= 0;

    // Get all legal moves
    let mut mvs: [(u64, u64, usize, usize); 300] = [(0,0,0,0); 300];
    let mvcnt ;

    // Qsearch
    if qsearch {
        mvcnt = moves::loudmoves(&mut mvs);

        let stand_pat = eval::evaluate();

        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }
    } else {
        mvcnt = moves::legalmoves(&mut mvs);
    }

    sorting::sort(&mut mvs);

    // Main Search
    let mut best = -99999999;

    for i in 0..mvcnt {

        if timer.elapsed().as_millis() >= unsafe {SEARCH_MS} && depth > 2{
            return 99999999;
        }

        let mv = mvs[i as usize];

        board::movebb(mv.0, mv.1, mv.2, mv.3);

        let eval = -search(depth-1, false, -beta, -alpha, ply+1, timer);
        board::undo();

        if eval > best {
            best = eval;

            unsafe {
                if root { SEARCH_BEST_MOVE = mv;SEARCH_EVAL=eval }
            }

            // Alpha beta pruning (~gazillion elo)
            if eval > alpha {alpha = eval;}
            if alpha >= beta {break;}
        }
    }

    if root {
        unsafe {
            ROOT_BEST_MOVE = SEARCH_BEST_MOVE;
            ROOT_EVAL = SEARCH_EVAL;
        }
    }

    if mvcnt == 0 {
        if qsearch {
            return eval::evaluate();
        }
        if board::in_check() {
            return ply - 999999;
        }
        return 0;
    }
    best
}

pub fn bestmove(search_time: u128) -> (u64, u64, usize, usize) {
    // Do time stuff
    let timer = Instant::now();
    unsafe {
        SEARCH_MS = search_time;
        // Set root eval to 0
        // If we keep it, there's a bug with mate
        ROOT_EVAL = 0;
    };


    // Iterative Deepening
    let mut i = 1;
    loop {
        search(i, true, -999999, 999999, 0, timer);
        println!("info depth {} score cp {} pv {}", i, unsafe { ROOT_EVAL }, unsafe { board::move_to_chess(ROOT_BEST_MOVE) });

        if unsafe {timer.elapsed().as_millis() >= SEARCH_MS || ROOT_EVAL > 500000} {
            break;
        }
        i += 1;

    }
    unsafe { ROOT_BEST_MOVE }
}