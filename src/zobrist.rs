use crate::bitloop;
pub use crate::board::*;

impl BoardState {
    pub fn zobrist_hash(&self, color: Color) -> u64 {
        let mut hash: u64 = 0;

        let bbs = self.bitboards();
        for i in 0..bbs.len() {
            if num_to_piece(i) == PieceType::WhitePieces || num_to_piece(i) == PieceType::BlackPieces {
                continue;
            }

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
        self.set_zobrist_hash(self.state().zobrist_hash(self.color()));
    }
    pub fn update_zobrist_hash(&mut self, square: u64, piece_type: PieceType) {
        self.set_zobrist_hash(self.zobrist_hash() ^ RANDS[(piece_type as usize) * 64 + square.trailing_zeros() as usize]);
    }
    pub fn update_zobrist_hash_square(&mut self, square: u64) {
        for pt in PieceType::WhitePawn as usize..PieceType::BlackKing as usize {
            if self.get_bitboard(num_to_piece(pt)) & square > 0 {
                self.update_zobrist_hash(square, num_to_piece(pt));
            }
        }
    }
    pub fn update_zobrist_hash_castle_rights(&mut self) {
        let bb = self.get_bitboard(PieceType::CastleRights);
        bitloop!(bb {
            self.set_zobrist_hash(self.zobrist_hash() ^ RANDS[64 * PieceType::CastleRights as usize + bb.trailing_zeros() as usize]);
        });
    }
    pub fn update_zobrist_hash_en_passant(&mut self) {
        let bb = self.get_bitboard(PieceType::EnPassant);
        bitloop!(bb {
            self.set_zobrist_hash(self.zobrist_hash() ^ RANDS[64 * PieceType::EnPassant as usize + bb.trailing_zeros() as usize]);
        });
    }
    pub fn update_zobrist_color(&mut self) {
        self.set_zobrist_hash(self.zobrist_hash() ^ RANDS[1025 - self.color() as usize])
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