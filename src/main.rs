pub use scarlet::board;
pub use scarlet::moves;
pub use scarlet::tools;
//use scarlet::tools::perft_lim;

use std::time::Instant;
//use std::arch::asm;

fn main() {
    let start = Instant::now();
    
    println!("{}", tools::perft_depth(6, false));

    println!("Time: {:?}", start.elapsed());
}

