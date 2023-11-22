pub use crate::board::*;
pub use crate::utils::*;

impl Board {
    pub fn perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        
        let mut moves = MoveList::new();
        self.gen_legal_moves(&mut moves);

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i];
            
            self.make_move(&mv);

            let subcount = self.sub_perft(depth - 1);

            self.undo();

            count += subcount;
            
            println!("{}: {}", move_to_chess(mv), subcount);
        }
        println!("nodes: {}", count);
        count
    }
    pub fn sub_perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.gen_legal_moves(&mut moves);

        if depth == 1 {
            return moves.pos as u64;
        }

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i];
            
            self.make_move(&mv);
            
            count += self.sub_perft(depth - 1);

            self.undo();
        }
        count
    }
}