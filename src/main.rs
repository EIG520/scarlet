pub use scarlet::board::*;
pub use scarlet::search::*;
use scarlet::uci::*;
// use std::time::Instant;

pub fn main() {
    let mut uci: UciHandler = UciHandler::new();

    // let now = Instant::now();

    // let _ = test_movegen_on_suite("C:\\Users\\eguer\\OneDrive\\Desktop\\Chess\\scarlet\\src\\ethereal_suite.epd");

    // println!("Elapsed time: {:?}", now.elapsed());

    uci.uci();
}