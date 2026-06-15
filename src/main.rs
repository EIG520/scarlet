pub use scarlet::board::*;
pub use scarlet::search::*;
use std::env;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let mut uci: UciHandler = UciHandler::new();

    // let _ = uci.handle_once(&mut "go wtime 100000".split_whitespace());

    let mut board = Board::new();
    let mut mvs = MoveList::default();
    board.gen_legal_moves(&mut mvs, true);

    for m in mvs.moves {
        println!("{}", move_to_chess(m));
    }

    uci.uci();
}