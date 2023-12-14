use crate::board::*;
use PieceType::*;
use Flag::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: u64,
    pub to: u64,
    pub piece_type: PieceType,
    pub flag: Flag,
}
impl Move {
    pub const fn null() -> Self {
        Self {from: 0, to: 0, piece_type: WhitePawn, flag: NoFlag}
    }
}