pub use scarlet::board::*;
pub use scarlet::search::*;
use scarlet::uci::*;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();

    uci.uci();
}