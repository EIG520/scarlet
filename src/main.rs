pub use scarlet::board::*;
pub use scarlet::search::*;
use std::env;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let mut uci: UciHandler = UciHandler::new();

    uci.uci();
}