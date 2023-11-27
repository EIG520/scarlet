pub use scarlet::board::*;
pub use scarlet::search::*;
use scarlet::uci::*;
// use std::time::Instant;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();
    
    let now = Instant::now();

    let _ = uci.handle_once(&mut "go depth 7".split_whitespace());

    println!("Elapsed time: {:?}", now.elapsed());

    // uci.uci();
}