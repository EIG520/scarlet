pub use crate::board::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fail {
    NoFail,
    FailHigh,
    FailLow,
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
            fail: Fail::NoFail,
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
        Self { depth: t.depth, score: t.score, best_move: t.best_move.long_form(), fail: t.fail }
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
            flag: mv.flag
        }
    }
    const fn empty() -> Self {
        CompactMove { data: 0, flag: Flag::NoFlag }
    }
}

pub struct TranspositionTable {
    table: Vec<Transposition>
}
impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self { 
            table: vec![Transposition::empty(); size]
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
                best_move: CompactMove::from(best_move)
            }
        }
    }
    pub fn probe(&self, board: &Board) -> Option<TranspositionInfo> {
        if self.table.len() > 0 { 
            let idx = (board.zobrist_hash() % (self.table.len() as u64)) as usize;
            let entry = self.table[idx];
            if board.zobrist_hash() >> 32 == entry.zobrist_leftbits as u64 && entry != Transposition::empty() {
                return Some(TranspositionInfo::from(entry));
            }
        }
        None
    }
    pub fn resize(&mut self, new_size: usize) {
        self.table = vec![Transposition::empty(); new_size];
    }
}