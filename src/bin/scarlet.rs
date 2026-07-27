pub use scarlet::board::*;
pub use scarlet::search::*;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();

    uci.uci();
}