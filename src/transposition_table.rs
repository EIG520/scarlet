pub use crate::board::*;
use crate::uci::SearchStackEntry;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fail {
    NoFail,
    FailHigh,
    FailLow,
    None,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Transposition {
    zobrist_leftbits: u32,
    score: i32,
    best_move: CompactMove,
    depth: i8,
    fail: Fail,
}
impl Transposition {
    const fn empty() -> Self {
        Transposition {
            zobrist_leftbits: 0,
            depth: 0,
            score: 1,
            fail: Fail::None,
            best_move: CompactMove::empty(),
        }
    }
    pub fn check_zobrist(&self, hash: u64) -> bool {
        return self.zobrist_leftbits == (hash >> 32) as u32;
    }
}

#[derive(Clone, Copy)]
pub struct TranspositionInfo {
    pub depth: i8,
    pub score: i32,
    pub best_move: Move,
    pub fail: Fail,
}
impl TranspositionInfo {
    pub fn from(t: Transposition) -> Self {
        Self {
            depth: t.depth,
            score: t.score,
            best_move: t.best_move.long_form(),
            fail: t.fail,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct CompactMove {
    data: u16,
    flag: Flag,
}
impl CompactMove {
    pub fn long_form(&self) -> Move {
        Move {
            from: 1 << ((self.data & 0b111111) as u64),
            to: 1 << (((self.data >> 6) & 0b111111) as u64),
            piece_type: num_to_piece((self.data >> 12) as usize),
            flag: self.flag,
        }
    }
    pub fn from(mv: Move) -> Self {
        CompactMove {
            data: mv.from.trailing_zeros() as u16
                | (mv.to.trailing_zeros() << 6) as u16
                | ((mv.piece_type as i32) << 12) as u16,
            flag: mv.flag,
        }
    }
    const fn empty() -> Self {
        CompactMove {
            data: 0,
            flag: Flag::NoFlag,
        }
    }
}

pub struct TranspositionTable {
    table: Vec<Transposition>,
}
impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![Transposition::empty(); size],
        }
    }
    pub fn add(&mut self, board: &Board, depth: i8, score: i32, best_move: Move, fail: Fail) {
        let len = self.table.len() as u64;
        if len > 0 {
            self.table[(board.zobrist_hash() % len) as usize] = Transposition {
                zobrist_leftbits: (board.zobrist_hash() >> 32) as u32,
                depth,
                score,
                fail,
                best_move: CompactMove::from(best_move),
            }
        }
    }
    pub fn probe(&self, board: &Board) -> Option<TranspositionInfo> {
        let len = self.table.len() as u64;
        if len > 0 {
            let entry = self.table[(board.zobrist_hash() % len) as usize];
            if (board.zobrist_hash() >> 32) as u32 == entry.zobrist_leftbits
                && entry.fail != Fail::None
            {
                return Some(TranspositionInfo::from(entry));
            }
        }
        None
    }
    pub fn resize(&mut self, new_size: usize) {
        self.table = vec![Transposition::empty(); new_size];
    }
}

pub struct HistoryTable {
    data: [[[i32; 64]; 64]; 12],
    killers: [Move; 100],

    capthist: Box<[[[i32; 64]; 64]; 12]>,
    cont_1ply: Box<[[[[i32; 64]; 12]; 64]; 12]>,
    // cont_2ply: Box<[[[[i32; 64]; 12]; 64]; 12]>
}

impl HistoryTable {
    pub fn probe(&self, ss: &[SearchStackEntry], mv: Move, ply: usize) -> i32 {
        let pmv = if ply > 0 && let Some(m) = ss.get(ply - 1) {
            m.mv
        } else {
            Move::null()
        };
        // let ppmv = if let Some(m) = ssrev.next() { m.mv } else { Move::null() };

        self.data[mv.piece_type as usize][mv.from.trailing_zeros() as usize]
            [mv.to.trailing_zeros() as usize]
            + if pmv != Move::null() {
                self.cont_1ply[pmv.piece_type as usize][pmv.to.trailing_zeros() as usize]
                    [mv.piece_type as usize][mv.to.trailing_zeros() as usize]
            } else {
                0
            }
        // + if ppmv != Move::null() { self.cont_2ply[ppmv.piece_type as usize][ppmv.to.trailing_zeros() as usize][mv.piece_type as usize][mv.to.trailing_zeros() as usize] } else { 0 }
    }

    pub fn probe_conthist(&self, ss: &[SearchStackEntry], mv: Move, ply: usize) -> i32 {
        let pmv = if let Some(m) = ss.get(ply - 1) {
            m.mv
        } else {
            Move::null()
        };

        if pmv != Move::null() {
            self.cont_1ply[pmv.piece_type as usize][pmv.to.trailing_zeros() as usize]
                [mv.piece_type as usize][mv.to.trailing_zeros() as usize]
        } else {
            0
        }
    }

    pub fn probe_tactical(&self, mv: Move, pt: usize) -> i32 {
        self.capthist[pt][mv.from.trailing_zeros() as usize][mv.to.trailing_zeros() as usize]
    }

    pub fn apply_delta(&mut self, ss: &[SearchStackEntry], mv: Move, delta: i32, ply: usize) {
        let deltac = delta.clamp(-512, 512);

        let pmv = if ply > 0 && let Some(m) = ss.get(ply - 1) {
            m.mv
        } else {
            Move::null()
        };
        // let ppmv = if let Some(m) = ssrev.next() { m.mv } else { Move::null() };

        self.data[mv.piece_type as usize][mv.from.trailing_zeros() as usize]
            [mv.to.trailing_zeros() as usize] += deltac
            - self.data[mv.piece_type as usize][mv.from.trailing_zeros() as usize]
                [mv.to.trailing_zeros() as usize]
                * deltac.abs()
                / 512;

        let contsum = if pmv != Move::null() {
            self.cont_1ply[pmv.piece_type as usize][pmv.to.trailing_zeros() as usize]
                [mv.piece_type as usize][mv.to.trailing_zeros() as usize]
        } else {
            0
        };
        // + if ppmv != Move::null() { self.cont_2ply[ppmv.piece_type as usize][ppmv.to.trailing_zeros() as usize][mv.piece_type as usize][mv.to.trailing_zeros() as usize] } else { 0 };

        if pmv != Move::null() {
            self.cont_1ply[pmv.piece_type as usize][pmv.to.trailing_zeros() as usize]
                [mv.piece_type as usize][mv.to.trailing_zeros() as usize] +=
                deltac - contsum * deltac.abs() / 512;
        }

        // if ppmv != Move::null() {
        //     self.cont_2ply[ppmv.piece_type as usize][ppmv.to.trailing_zeros() as usize][mv.piece_type as usize][mv.to.trailing_zeros() as usize] +=
        //             deltac - contsum * deltac.abs() / 512;
        // }
    }

    pub fn apply_delta_tactical(&mut self, mv: Move, pt: usize, delta: i32) {
        let deltac = delta.clamp(-512, 512);

        self.capthist[pt][mv.from.trailing_zeros() as usize][mv.to.trailing_zeros() as usize] +=
            deltac
                - self.capthist[pt][mv.from.trailing_zeros() as usize]
                    [mv.to.trailing_zeros() as usize]
                    * deltac.abs()
                    / 512;
    }

    pub fn add_killer(&mut self, mv: Move, ply: i32) {
        self.killers[ply.clamp(0, self.killers.len() as i32 - 1) as usize] = mv;
    }

    pub fn get_killer(&self, ply: i32) -> Move {
        self.killers[ply.clamp(0, self.killers.len() as i32 - 1) as usize]
    }
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self {
            data: [[[0; 64]; 64]; 12],
            killers: [Move::null(); 100],
            cont_1ply: unsafe { Box::<_>::new_zeroed().assume_init() },
            // cont_2ply: unsafe { Box::<_>::new_zeroed().assume_init() },
            capthist: unsafe { Box::<_>::new_zeroed().assume_init() },
        }
    }
}
