use crate::bitloop;
pub use crate::board::*;

impl Board {
    pub fn evaluate(&mut self) {
        // let mut eg_eval = 0;
        // let mut mg_eval = 0;

        // // Account for kings
        // // I could just have a phase table but shut up
        // let mut phase: i32 = -12;

        // for i in 0..6 {
        //     let mut bb = self.get_bitboard(num_to_piece(2*i));
        //     while bb > 0 {
        //         let pos = bb.blsi().trailing_zeros() ^ 7;

        //         mg_eval += EVAL_TABLES[2*i][(pos ^ 56) as usize] + MG_PV[i];

        //         eg_eval += EVAL_TABLES[2*i+1][(pos ^ 56) as usize] + EG_PV[i];

        //         phase += i as i32;


        //         bb = bb & bb-1;
        //     }
        //     let mut bb = self.get_bitboard(num_to_piece(2*i+1));
        //     while bb > 0 {
        //         let pos = bb.blsi().trailing_zeros() ^ 7;

        //         mg_eval -= EVAL_TABLES[2*i][pos as usize] + MG_PV[i];
        //         eg_eval -= EVAL_TABLES[2*i+1][pos as usize] + EG_PV[i];


        //         phase += i as i32;


        //         bb = bb & (bb-1);
        //     }

        // }
        // self.set_eg_eval(eg_eval);
        // self.set_mg_eval(mg_eval);
        // self.set_phase(phase);
    }
    pub fn update_eval(&mut self, piece_type: PieceType, from: u64, to: u64) {
        let offset = 384 * (piece_type as usize & 1) + (piece_type as usize >> 1) * 64;
        let acc = self.get_waccum_mut();
        let weightsf = NNUE.feature_weights[offset + (from.trailing_zeros() ^ 7) as usize];
        let weightst = NNUE.feature_weights[offset + (to.trailing_zeros() ^ 7) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weightsf.vals[i];
            acc.vals[i] += weightst.vals[i];
        }

        let offset = 384 * (!(piece_type as usize) & 1) + (piece_type as usize >> 1) * 64;
        let acc = self.get_baccum_mut();
        let weightsf = NNUE.feature_weights[offset + (from.trailing_zeros() ^ 63) as usize];
        let weightst = NNUE.feature_weights[offset + (to.trailing_zeros() ^ 63) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weightsf.vals[i];
            acc.vals[i] += weightst.vals[i];
        }
    }
    pub fn update_eval_capture(&mut self, piece_type: PieceType, square: u64) {
        let offset = 384 * (piece_type as usize & 1) + (piece_type as usize >> 1) * 64;
        let acc = self.get_waccum_mut();
        let weights = NNUE.feature_weights[offset + (square.trailing_zeros() ^ 7) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weights.vals[i];
        }

        let offset = 384 * (!(piece_type as usize) & 1) + (piece_type as usize >> 1) * 64;
        let acc = self.get_baccum_mut();
        let weights = NNUE.feature_weights[offset + (square.trailing_zeros() ^ 63) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weights.vals[i];
        }
    }
    pub fn update_eval_promotion(&mut self, promotion_type: PieceType, from: u64, to: u64) {
        let offset_promo = 384 * (promotion_type as usize & 1) + (promotion_type as usize >> 1) * 64;
        let offset_pawn = 384 * (promotion_type as usize & 1);
        let acc = self.get_waccum_mut();
        let weightsf = NNUE.feature_weights[offset_pawn + (from.trailing_zeros() ^ 7) as usize];
        let weightst = NNUE.feature_weights[offset_promo + (to.trailing_zeros() ^ 7) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weightsf.vals[i];
            acc.vals[i] += weightst.vals[i];
        }

        let offset_promo = 384 * (!(promotion_type as usize) & 1) + (promotion_type as usize >> 1) * 64;
        let offset_pawn = 384 * (!(promotion_type as usize) & 1);
        let acc = self.get_baccum_mut();
        let weightsf = NNUE.feature_weights[offset_pawn + (from.trailing_zeros() ^ 63) as usize];
        let weightst = NNUE.feature_weights[offset_promo + (to.trailing_zeros() ^ 63) as usize];

        for i in 0..HIDDEN {
            acc.vals[i] -= weightsf.vals[i];
            acc.vals[i] += weightst.vals[i];
        }


        // self.set_phase(self.phase() + promotion_type as i32 / 2);
        // match self.color() {
        //     Color::White => {
        //         self.set_mg_eval(self.mg_eval() - EVAL_TABLES[PieceType::WhitePawn as usize][from.trailing_zeros() as usize ^ 63] - MG_PV[PieceType::WhitePawn as usize / 2]);
        //         self.set_eg_eval(self.eg_eval() - EVAL_TABLES[PieceType::WhitePawn as usize + 1][from.trailing_zeros() as usize ^ 63] - EG_PV[PieceType::WhitePawn as usize / 2]);

        //         self.set_mg_eval(self.mg_eval() + EVAL_TABLES[promotion_type as usize][to.trailing_zeros() as usize ^ 63] + MG_PV[promotion_type as usize / 2]);
        //         self.set_eg_eval(self.eg_eval() + EVAL_TABLES[promotion_type as usize + 1][to.trailing_zeros() as usize ^  63] + EG_PV[promotion_type as usize / 2]);
        //     }
        //     Color::Black => {
        //         self.set_mg_eval(self.mg_eval() + EVAL_TABLES[PieceType::BlackPawn as usize - 1][from.trailing_zeros() as usize ^ 7] + MG_PV[PieceType::BlackPawn as usize / 2]);
        //         self.set_eg_eval(self.eg_eval() + EVAL_TABLES[PieceType::BlackPawn as usize][from.trailing_zeros() as usize ^ 7] + EG_PV[PieceType::BlackPawn as usize / 2]);
        //         // println!("{}", EVAL_TABLES[PieceType::BlackPawn as usize - 1][from.trailing_zeros() as usize ^ 7]);
        //         // println!("{}", from);



        //         self.set_mg_eval(self.mg_eval() - EVAL_TABLES[promotion_type as usize - 1][to.trailing_zeros() as usize ^ 7] - MG_PV[promotion_type as usize / 2]);
        //         self.set_eg_eval(self.eg_eval() - EVAL_TABLES[promotion_type as usize][to.trailing_zeros() as usize ^ 7] - EG_PV[promotion_type as usize / 2]);
        //     }
        // }
    }
}

const HIDDEN: usize = 256;
const SCALE: i32 = 400;
const QA: i16 = 255;
const QB: i16 = 64;

static NNUE: Network =
    unsafe { std::mem::transmute(*include_bytes!(r"net_witch_256.bin")) };

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Accumulator {
    vals: [i16; HIDDEN],
}

impl Default for Accumulator {
    fn default() -> Self {
        Accumulator { vals: [0; HIDDEN] }
    }
}

#[repr(C, align(64))]
pub struct Network {
    feature_weights: [Accumulator; 768],
    feature_bias: Accumulator,
    output_weights: [i16; 2 * HIDDEN],
    output_bias: i16,
}

fn screlu(x: i16) -> i32 {
    let y = i32::from(x).clamp(0, i32::from(QA));
    y * y
}

impl Board {
    pub fn populate_accumulators(&mut self) {
        let wpieces = [self.get_bitboard(PieceType::WhitePawn), self.get_bitboard(PieceType::WhiteKnight), self.get_bitboard(PieceType::WhiteBishop), self.get_bitboard(PieceType::WhiteRook), self.get_bitboard(PieceType::WhiteQueen), self.get_bitboard(PieceType::WhiteKing)];
        let bpieces = [self.get_bitboard(PieceType::BlackPawn), self.get_bitboard(PieceType::BlackKnight), self.get_bitboard(PieceType::BlackBishop), self.get_bitboard(PieceType::BlackRook), self.get_bitboard(PieceType::BlackQueen), self.get_bitboard(PieceType::BlackKing)];
        
        // Do white accumulator
        let mut acc = [0; HIDDEN];
        for i in 0..6 {
            let bb = wpieces[i];

            bitloop!(bb {
                let pos = (bb.blsi().trailing_zeros() ^ 7) as usize;
                let weights = NNUE.feature_weights[64 * i + pos];

                for j in 0..HIDDEN {
                    acc[j] += weights.vals[j];
                }
            });

            bb = bpieces[i];
            bitloop!(bb {
                let pos = (bb.blsi().trailing_zeros() ^ 7) as usize;
                let weights = NNUE.feature_weights[384 + 64 * i + pos];

                for j in 0..HIDDEN {
                    acc[j] += weights.vals[j];
                }
            });
        }

        for j in 0..HIDDEN {
            acc[j] += NNUE.feature_bias.vals[j];
        }

        self.set_waccum(Accumulator { vals: acc });

        // Do black accumulator
        acc = [0; HIDDEN];
        for i in 0..6 {
            let bb = bpieces[i];

            bitloop!(bb {
                let pos = (bb.blsi().trailing_zeros() ^ 63) as usize;
                let weights = NNUE.feature_weights[64 * i + pos];

                for j in 0..HIDDEN {
                    acc[j] += weights.vals[j];
                }
            });

            bb = wpieces[i];
            bitloop!(bb {
                let pos = (bb.blsi().trailing_zeros() ^ 63) as usize;
                let weights = NNUE.feature_weights[384 + 64 * i + pos];

                for j in 0..HIDDEN {
                    acc[j] += weights.vals[j];
                }
            });
        }

        for j in 0..HIDDEN {
            acc[j] += NNUE.feature_bias.vals[j];
        }

        self.set_baccum(Accumulator { vals: acc });

    }

    pub fn eval(&self) -> i32 {
        let stm_accum = if self.color() == Color::White { self.get_waccum() } else { self.get_baccum() };
        let ntm_accum = if self.color() == Color::White { self.get_baccum() } else { self.get_waccum() };

        let mut output = 0;

        for i in 0..HIDDEN {
            output += screlu(stm_accum.vals[i]) * NNUE.output_weights[i] as i32;
            output += screlu(ntm_accum.vals[i]) * NNUE.output_weights[HIDDEN + i] as i32;
        }

        output /= QA as i32;

        output += NNUE.output_bias as i32;

        output *= SCALE;

        output /= QA as i32 * QB as i32;

        output
    }

}