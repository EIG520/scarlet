use crate::bitloop;
pub use crate::board::*;

impl BoardState {
    pub fn zobrist_hash(&self, color: Color) -> u64 {
        let mut hash: u64 = 0;

        let bbs = self.bitboards();
        for i in 0..bbs.len() {
            let bb = bbs[i];
            bitloop!(bb{
                hash ^= RANDS[64 * i + bb.trailing_zeros() as usize];
            });
        }
        match color {
            Color::Black => hash ^= RANDS[1024],
            Color::White => hash ^= RANDS[1025]
        }

        hash
    }
}
impl Board {
    pub fn gen_zobrist_hash(&mut self) {
        self.zobrist_hash = self.state().zobrist_hash(self.color());
    }
    // TODO: incremental update
    pub fn update_zobrist_hash(&mut self) {
        self.zobrist_hash = self.state().zobrist_hash(self.color());
    }
}

const RAND_COUNT: usize = 1026;
const RANDS: [u64; RAND_COUNT] = gen_rands();

pub const fn gen_rands() -> [u64; RAND_COUNT] {
    let mut arr: [u64; RAND_COUNT] = [0; RAND_COUNT];
    let mut seed: u64 = 123456789;

    let mut i = 0;
    while i < RAND_COUNT {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        arr[i] = seed;
        i += 1;
    }
    arr
}