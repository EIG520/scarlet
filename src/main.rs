pub use scarlet::board::*;
pub use scarlet::search::*;
use scarlet::uci::*;
// use std::time::Instant;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();

    uci.uci();
}