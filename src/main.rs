pub use scarlet::board::*;
pub use scarlet::search::*;
use scarlet::uci::*;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();

    let _ = uci.handle_once(&mut "go wtime 100000".split_whitespace());

    // uci.uci();
}