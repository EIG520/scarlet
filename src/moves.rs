pub use crate::board;
pub use bitintr::*;
use std::arch::asm;

pub struct Move {
    pub from: usize,
    pub to: usize
}

// All special move flags:
// 0: Nothing special
// 1: en passant
// 2: en passant (black)
// 3: promotion (knight)
// 4: promotion (bishop)
// 5: promotion (rook)
// 6: promotion (queen)
// 7: castle-kingside
// 8: castle-queenside
// 9: castle-kingside (black)
// 10: castle-queenside (black)

// I used this to calculate knight_bbmoves
pub fn knight_moves(square: usize) -> Vec<Move>{
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;
    
    // Handle moves which change y by one
    if y != 0 {
        if x > 1 {
            moves.push(Move {from: square, to: square - 10});
        }
        if x < 6 {
            moves.push(Move {from: square, to: square - 6});
        }
    }
    if y != 7 {
        if x > 1 {
            moves.push(Move {from: square, to: square + 6});
        }
        if x < 6 {
            moves.push(Move {from: square, to: square + 10});
        }
    }
    // Handle moves which change x by one
    if x != 0 {
        if y < 6 {
            moves.push(Move {from: square, to: square + 15});
        }
        if y > 1 {
            moves.push(Move {from: square, to: square - 17})
        }
    }
    if x != 7 {
        if y < 6 {
            moves.push(Move {from: square, to: square + 17});
        }
        if y > 1 {
            moves.push(Move {from: square, to: square - 15});
        }
    }

    return moves;
}

// I used this to calculate king_bbmoves
pub fn king_moves(square: usize) -> Vec<Move>{
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if x < 7 {moves.push( Move {from: square, to: square + 1} )}
    if x > 0 {moves.push( Move {from: square, to: square - 1} )}
    if y < 7 {moves.push( Move {from: square, to: square + 8} )}
    if y > 0 {moves.push( Move {from: square, to: square - 8} )}
    if x < 7 && y < 7 {moves.push( Move {from: square, to: square + 9} )}
    if x < 7 && y > 0 {moves.push( Move {from: square, to: square - 7} )}
    if x > 0 && y < 7 {moves.push( Move {from: square, to: square + 7} )}
    if x > 0 && y > 0 {moves.push( Move {from: square, to: square - 9} )}

    return moves;
}

// used to calculate pawn_bbmoves
pub fn white_pawn_moves(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if y < 7 {moves.push( Move {from: square, to: square + 8} )};

    return moves;
}
pub fn white_pawn_doub(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if y == 1 {moves.push( Move {from: square, to: square + 16})};

    return moves;
}
pub fn white_pawn_atk_moves(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if x < 7 && y < 7 {moves.push( Move {from: square, to: square + 9} )}
    if x > 0 && y < 7{moves.push( Move {from: square, to: square + 7} )}

    return moves;
}
pub fn black_pawn_moves(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if y > 0 {moves.push( Move {from: square, to: square - 8} )}
    //if y == 6 {moves.push( Move {from: square, to: square - 16})}

    return moves;
}
pub fn black_pawn_doub(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if y == 6 {moves.push( Move {from: square, to: square - 16})}

    return moves;
}
pub fn black_pawn_atk_moves(square: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    if x < 7 && y > 0 {moves.push( Move {from: square, to: square - 7} )}
    if x > 0 && y > 0 {moves.push( Move {from: square, to: square - 9} )}

    return moves;
}

pub fn rook_moves(square: usize) -> Vec<Vec<Move>> {
    let x: usize = square % 8;
    let _y: usize = (square - x) / 8;

    let mut moves1: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 > 0 {
        i -= 1;
        moves1.push( Move {from: square, to: i} );
    }
    let mut moves2: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 < 7 {
        i += 1;
        moves2.push( Move {from: square, to: i} );
    }

    let mut moves3: Vec<Move> = vec![];
    let mut i: usize = square;
    while i / 8 > 0 {
        i -= 8;
        moves3.push( Move {from: square, to: i} );
    }

    let mut moves4: Vec<Move> = vec![];
    let mut i: usize = square;
    while i / 8 < 7 {
        i += 8;
        moves4.push( Move {from: square, to: i} );
    }
    return vec![moves1, moves2, moves3, moves4];

}

pub fn bishop_moves(square: usize) -> Vec<Vec<Move>> {
    let x: usize = square % 8;
    let _y: usize = (square - x) / 8;

    let mut moves1: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 > 0 && i / 8 < 7 {
        i += 7;
        moves1.push( Move {from: square, to: i} );
    }

    let mut moves2: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 < 7 && i / 8 < 7 {
        i += 9;
        moves2.push( Move {from: square, to: i} );
    }

    let mut moves3: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 > 0 && i / 8 > 0 {
        i -= 9;
        moves3.push( Move {from: square, to: i} );
    }

    let mut moves4: Vec<Move> = vec![];
    let mut i: usize = square;
    while i % 8 < 7 && i / 8 > 0 {
        i -= 7;
        moves4.push( Move {from: square, to: i} );
    }

    return vec![moves1, moves2, moves3, moves4];
}


// Get all possible moves on square (checks, etc verified later)
const NMOVES: [u64; 64] = [0x0000000000020400,0x0000000000050800,0x00000000000a1100,0x0000000000142200,0x0000000000284400,0x0000000000508800,0x0000000000a01000,0x0000000000402000,0x0000000002040004,0x0000000005080008,0x000000000a110011,0x0000000014220022,0x0000000028440044,0x0000000050880088,0x00000000a0100010,0x0000000040200020,0x0000000204000402,0x0000000508000805,0x0000000a1100110a,0x0000001422002214,0x0000002844004428,0x0000005088008850,0x000000a0100010a0,0x0000004020002040,0x0000020400040200,0x0000050800080500,0x00000a1100110a00,0x0000142200221400,0x0000284400442800,0x0000508800885000,0x0000a0100010a000,0x0000402000204000,0x0002040004020000,0x0005080008050000,0x000a1100110a0000,0x0014220022140000,0x0028440044280000,0x0050880088500000,0x00a0100010a00000,0x0040200020400000,0x0204000402000000,0x0508000805000000,0x0a1100110a000000,0x1422002214000000,0x2844004428000000,0x5088008850000000,0xa0100010a0000000,0x4020002040000000,0x0400040200000000,0x0800080500000000,0x1100110a00000000,0x2200221400000000,0x4400442800000000,0x8800885000000000,0x100010a000000000,0x2000204000000000,0x0004020000000000,0x0008050000000000,0x00110a0000000000,0x0022140000000000,0x0044280000000000,0x0088500000000000,0x0010a00000000000,0x0020400000000000];
pub fn knight_bbmoves(square: usize) -> u64 {
    return NMOVES[square] & !board::get_bitboard(12 + board::color());
}

const KMOVES: [u64; 64] = [0x0000000000000302,0x0000000000000705,0x0000000000000e0a,0x0000000000001c14,0x0000000000003828,0x0000000000007050,0x000000000000e0a0,0x000000000000c040,0x0000000000030203,0x0000000000070507,0x00000000000e0a0e,0x00000000001c141c,0x0000000000382838,0x0000000000705070,0x0000000000e0a0e0,0x0000000000c040c0,0x0000000003020300,0x0000000007050700,0x000000000e0a0e00,0x000000001c141c00,0x0000000038283800,0x0000000070507000,0x00000000e0a0e000,0x00000000c040c000,0x0000000302030000,0x0000000705070000,0x0000000e0a0e0000,0x0000001c141c0000,0x0000003828380000,0x0000007050700000,0x000000e0a0e00000,0x000000c040c00000,0x0000030203000000,0x0000070507000000,0x00000e0a0e000000,0x00001c141c000000,0x0000382838000000,0x0000705070000000,0x0000e0a0e0000000,0x0000c040c0000000,0x0003020300000000,0x0007050700000000,0x000e0a0e00000000,0x001c141c00000000,0x0038283800000000,0x0070507000000000,0x00e0a0e000000000,0x00c040c000000000,0x0302030000000000,0x0705070000000000,0x0e0a0e0000000000,0x1c141c0000000000,0x3828380000000000,0x7050700000000000,0xe0a0e00000000000,0xc040c00000000000,0x0203000000000000,0x0507000000000000,0x0a0e000000000000,0x141c000000000000,0x2838000000000000,0x5070000000000000,0xa0e0000000000000,0x40c0000000000000];
pub fn king_bbmoves(square: usize) -> u64 {
    return KMOVES[square] & !board::get_bitboard(12 + board::color());
}

const FLAT_WPMOVES: [u64; 64] = [0x0000000000000100, 0x0000000000000200, 0x0000000000000400, 0x0000000000000800, 0x0000000000001000, 0x0000000000002000, 0x0000000000004000, 0x0000000000008000, 0x0000000000010000, 0x0000000000020000, 0x0000000000040000, 0x0000000000080000, 0x0000000000100000, 0x0000000000200000, 0x0000000000400000, 0x0000000000800000, 0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000, 0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000, 0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000, 0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000, 0x0000010000000000, 0x0000020000000000, 0x0000040000000000, 0x0000080000000000, 0x0000100000000000, 0x0000200000000000, 0x0000400000000000, 0x0000800000000000, 0x0001000000000000, 0x0002000000000000, 0x0004000000000000, 0x0008000000000000, 0x0010000000000000, 0x0020000000000000, 0x0040000000000000, 0x0080000000000000, 0x0100000000000000, 0x0200000000000000, 0x0400000000000000, 0x0800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000];
const DOUB_WPMOVES: [u64; 64] = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000, 0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000];
const ATK_WPMOVES: [u64; 64] = [0x0000000000000200, 0x0000000000000500, 0x0000000000000a00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000a000, 0x0000000000004000, 0x0000000000020000, 0x0000000000050000, 0x00000000000a0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000a00000, 0x0000000000400000, 0x0000000002000000, 0x0000000005000000, 0x000000000a000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000a0000000, 0x0000000040000000, 0x0000000200000000, 0x0000000500000000, 0x0000000a00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000a000000000, 0x0000004000000000, 0x0000020000000000, 0x0000050000000000, 0x00000a0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000a00000000000, 0x0000400000000000, 0x0002000000000000, 0x0005000000000000, 0x000a000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00a0000000000000, 0x0040000000000000, 0x0200000000000000, 0x0500000000000000, 0x0a00000000000000, 0x1400000000000000, 0x2800000000000000, 0x5000000000000000, 0xa000000000000000, 0x4000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000];
#[inline(always)]
pub fn wpawn_bbmoves(square: usize) -> u64 {
    return ((FLAT_WPMOVES[square] | DOUB_WPMOVES[square] & (!(board::get_bitboard(13) | board::get_bitboard(12)) << 8) ) & !board::get_bitboard(13) ) & !board::get_bitboard(12) | wpawnatk_bbmoves(square);
}
#[inline(always)]
pub fn wpawnatk_bbmoves(square: usize) -> u64 {
    return ATK_WPMOVES[square] & board::get_bitboard( 13 );
}
#[inline(always)]
pub fn wpawnep_bbmoves(square: usize) -> u64 {
    return ATK_WPMOVES[square] & (board::get_prev_bitboard( 1 ) >> 8) & ((board::get_bitboard( 1 ) << 8) ^ (board::get_bitboard( 1) >> 8)) & ((board::get_prev_bitboard( 1 ) << 8) ^ (board::get_prev_bitboard( 1 ) >> 8)) & (board::get_bitboard( 1 ) << 8);
}

const FLAT_BPMOVES: [u64; 64] = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000001, 0x0000000000000002, 0x0000000000000004, 0x0000000000000008, 0x0000000000000010, 0x0000000000000020, 0x0000000000000040, 0x0000000000000080, 0x0000000000000100, 0x0000000000000200, 0x0000000000000400, 0x0000000000000800, 0x0000000000001000, 0x0000000000002000, 0x0000000000004000, 0x0000000000008000, 0x0000000000010000, 0x0000000000020000, 0x0000000000040000, 0x0000000000080000, 0x0000000000100000, 0x0000000000200000, 0x0000000000400000, 0x0000000000800000, 0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000, 0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000, 0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000, 0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000, 0x0000010000000000, 0x0000020000000000, 0x0000040000000000, 0x0000080000000000, 0x0000100000000000, 0x0000200000000000, 0x0000400000000000, 0x0000800000000000, 0x0001000000000000, 0x0002000000000000, 0x0004000000000000, 0x0008000000000000, 0x0010000000000000, 0x0020000000000000, 0x0040000000000000, 0x0080000000000000];
const DOUB_BPMOVES: [u64; 64] = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000, 0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000];
const ATK_BPMOVES: [u64; 64] = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000002, 0x0000000000000005, 0x000000000000000a, 0x0000000000000014, 0x0000000000000028, 0x0000000000000050, 0x00000000000000a0, 0x0000000000000040, 0x0000000000000200, 0x0000000000000500, 0x0000000000000a00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000a000, 0x0000000000004000, 0x0000000000020000, 0x0000000000050000, 0x00000000000a0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000a00000, 0x0000000000400000, 0x0000000002000000, 0x0000000005000000, 0x000000000a000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000a0000000, 0x0000000040000000, 0x0000000200000000, 0x0000000500000000, 0x0000000a00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000a000000000, 0x0000004000000000, 0x0000020000000000, 0x0000050000000000, 0x00000a0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000a00000000000, 0x0000400000000000, 0x0002000000000000, 0x0005000000000000, 0x000a000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00a0000000000000, 0x0040000000000000];
#[inline(always)]
pub fn bpawn_bbmoves(square: usize) -> u64 {
    return ((FLAT_BPMOVES[square] | DOUB_BPMOVES[square] & (!(board::get_bitboard(12) | board::get_bitboard(13)) >> 8) ) & !board::get_bitboard(12) ) & !board::get_bitboard(13) | bpawnatk_bbmoves(square);
}
#[inline(always)]
pub fn bpawnatk_bbmoves(square: usize) -> u64 {
    return ATK_BPMOVES[square] & board::get_bitboard( 12 );
}
#[inline(always)]
pub fn bpawnep_bbmoves(square: usize) -> u64 {
    return ATK_BPMOVES[square] & (board::get_prev_bitboard( 0 ) << 8) & ((board::get_bitboard( 0 ) << 8) ^ (board::get_bitboard( 0 ) >> 8)) & ((board::get_prev_bitboard( 0 ) << 8) ^ (board::get_prev_bitboard( 0 ) >> 8)) & (board::get_bitboard( 0 ) >> 8);
}
#[inline(always)]
pub fn queen_bbmoves(square: usize) -> u64 {
    return rook_bbmoves(square) | bishop_bbmoves(square);
}

// Sliding Pieces

const RMOVES_RIGHT: [u64; 64] = [0x0000000000000000,0x0000000000000001,0x0000000000000003,0x0000000000000007,0x000000000000000f,0x000000000000001f,0x000000000000003f,0x000000000000007f,0x0000000000000000,0x0000000000000100,0x0000000000000300,0x0000000000000700,0x0000000000000f00,0x0000000000001f00,0x0000000000003f00,0x0000000000007f00,0x0000000000000000,0x0000000000010000,0x0000000000030000,0x0000000000070000,0x00000000000f0000,0x00000000001f0000,0x00000000003f0000,0x00000000007f0000,0x0000000000000000,0x0000000001000000,0x0000000003000000,0x0000000007000000,0x000000000f000000,0x000000001f000000,0x000000003f000000,0x000000007f000000,0x0000000000000000,0x0000000100000000,0x0000000300000000,0x0000000700000000,0x0000000f00000000,0x0000001f00000000,0x0000003f00000000,0x0000007f00000000,0x0000000000000000,0x0000010000000000,0x0000030000000000,0x0000070000000000,0x00000f0000000000,0x00001f0000000000,0x00003f0000000000,0x00007f0000000000,0x0000000000000000,0x0001000000000000,0x0003000000000000,0x0007000000000000,0x000f000000000000,0x001f000000000000,0x003f000000000000,0x007f000000000000,0x0000000000000000,0x0100000000000000,0x0300000000000000,0x0700000000000000,0x0f00000000000000,0x1f00000000000000,0x3f00000000000000,0x7f00000000000000];
const RMOVES_LEFT: [u64; 64] = [0x00000000000000fe,0x00000000000000fc,0x00000000000000f8,0x00000000000000f0,0x00000000000000e0,0x00000000000000c0,0x0000000000000080,0x0000000000000000,0x000000000000fe00,0x000000000000fc00,0x000000000000f800,0x000000000000f000,0x000000000000e000,0x000000000000c000,0x0000000000008000,0x0000000000000000,0x0000000000fe0000,0x0000000000fc0000,0x0000000000f80000,0x0000000000f00000,0x0000000000e00000,0x0000000000c00000,0x0000000000800000,0x0000000000000000,0x00000000fe000000,0x00000000fc000000,0x00000000f8000000,0x00000000f0000000,0x00000000e0000000,0x00000000c0000000,0x0000000080000000,0x0000000000000000,0x000000fe00000000,0x000000fc00000000,0x000000f800000000,0x000000f000000000,0x000000e000000000,0x000000c000000000,0x0000008000000000,0x0000000000000000,0x0000fe0000000000,0x0000fc0000000000,0x0000f80000000000,0x0000f00000000000,0x0000e00000000000,0x0000c00000000000,0x0000800000000000,0x0000000000000000,0x00fe000000000000,0x00fc000000000000,0x00f8000000000000,0x00f0000000000000,0x00e0000000000000,0x00c0000000000000,0x0080000000000000,0x0000000000000000,0xfe00000000000000,0xfc00000000000000,0xf800000000000000,0xf000000000000000,0xe000000000000000,0xc000000000000000,0x8000000000000000,0x0000000000000000];
const RMOVES_UP: [u64; 64] = [0x0101010101010100,0x0202020202020200,0x0404040404040400,0x0808080808080800,0x1010101010101000,0x2020202020202000,0x4040404040404000,0x8080808080808000,0x0101010101010000,0x0202020202020000,0x0404040404040000,0x0808080808080000,0x1010101010100000,0x2020202020200000,0x4040404040400000,0x8080808080800000,0x0101010101000000,0x0202020202000000,0x0404040404000000,0x0808080808000000,0x1010101010000000,0x2020202020000000,0x4040404040000000,0x8080808080000000,0x0101010100000000,0x0202020200000000,0x0404040400000000,0x0808080800000000,0x1010101000000000,0x2020202000000000,0x4040404000000000,0x8080808000000000,0x0101010000000000,0x0202020000000000,0x0404040000000000,0x0808080000000000,0x1010100000000000,0x2020200000000000,0x4040400000000000,0x8080800000000000,0x0101000000000000,0x0202000000000000,0x0404000000000000,0x0808000000000000,0x1010000000000000,0x2020000000000000,0x4040000000000000,0x8080000000000000,0x0100000000000000,0x0200000000000000,0x0400000000000000,0x0800000000000000,0x1000000000000000,0x2000000000000000,0x4000000000000000,0x8000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000];
const RMOVES_DOWN: [u64; 64] = [0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000001,0x0000000000000002,0x0000000000000004,0x0000000000000008,0x0000000000000010,0x0000000000000020,0x0000000000000040,0x0000000000000080,0x0000000000000101,0x0000000000000202,0x0000000000000404,0x0000000000000808,0x0000000000001010,0x0000000000002020,0x0000000000004040,0x0000000000008080,0x0000000000010101,0x0000000000020202,0x0000000000040404,0x0000000000080808,0x0000000000101010,0x0000000000202020,0x0000000000404040,0x0000000000808080,0x0000000001010101,0x0000000002020202,0x0000000004040404,0x0000000008080808,0x0000000010101010,0x0000000020202020,0x0000000040404040,0x0000000080808080,0x0000000101010101,0x0000000202020202,0x0000000404040404,0x0000000808080808,0x0000001010101010,0x0000002020202020,0x0000004040404040,0x0000008080808080,0x0000010101010101,0x0000020202020202,0x0000040404040404,0x0000080808080808,0x0000101010101010,0x0000202020202020,0x0000404040404040,0x0000808080808080,0x0001010101010101,0x0002020202020202,0x0004040404040404,0x0008080808080808,0x0010101010101010,0x0020202020202020,0x0040404040404040,0x0080808080808080];

#[inline(always)]
pub fn rook_bbmoves(square: usize) -> u64 {
    let main_bb: u64 = board::get_bitboard(12) | board::get_bitboard(13);

    // Right
    let mut right: u64 = main_bb;

    // have to do this since bitintr pext takes like 10 clock cycles (ew)
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            a = inout(reg) right,
            b = in(reg) RMOVES_RIGHT[square]
        )
    }

    // asm probably has a reverse bits command, but idk it so for now I'll just use bitintr since it is fast enough
    right = right.reverse_bits().blsmsk().reverse_bits();
    unsafe {
        asm!(
            "pdep {a}, {a}, {b}",
            a = inout(reg) right,
            b = in(reg) RMOVES_RIGHT[square]
        )
    }

    // Left
    let mut left: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) left,
            b = in(reg) RMOVES_LEFT[square]
        )
    }

    // Top
    let mut up: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) up,
            b = in(reg) RMOVES_UP[square]
        )
    }

    // Bottom
    let mut down: u64 = main_bb;
    // have to do this since bitintr pext takes like 10 clock cycles (ew)
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            a = inout(reg) down,
            b = in(reg) RMOVES_DOWN[square]
        )
    }

    // asm probably has a reverse bits command, but idk it so for now I'll just use bitintr since it is fast enough
    down = down.reverse_bits().blsmsk().reverse_bits();
    unsafe {
        asm!(
            "pdep {a}, {a}, {b}",
            a = inout(reg) down,
            b = in(reg) RMOVES_DOWN[square]
        )
    }

    return (right | left | up | down) & !board::get_bitboard(12 + board::color());
}

const BMOVES_UR: [u64; 64] = [0x0000000000000000,0x0000000000000100,0x0000000000010200,0x0000000001020400,0x0000000102040800,0x0000010204081000,0x0001020408102000,0x0102040810204000,0x0000000000000000,0x0000000000010000,0x0000000001020000,0x0000000102040000,0x0000010204080000,0x0001020408100000,0x0102040810200000,0x0204081020400000,0x0000000000000000,0x0000000001000000,0x0000000102000000,0x0000010204000000,0x0001020408000000,0x0102040810000000,0x0204081020000000,0x0408102040000000,0x0000000000000000,0x0000000100000000,0x0000010200000000,0x0001020400000000,0x0102040800000000,0x0204081000000000,0x0408102000000000,0x0810204000000000,0x0000000000000000,0x0000010000000000,0x0001020000000000,0x0102040000000000,0x0204080000000000,0x0408100000000000,0x0810200000000000,0x1020400000000000,0x0000000000000000,0x0001000000000000,0x0102000000000000,0x0204000000000000,0x0408000000000000,0x0810000000000000,0x1020000000000000,0x2040000000000000,0x0000000000000000,0x0100000000000000,0x0200000000000000,0x0400000000000000,0x0800000000000000,0x1000000000000000,0x2000000000000000,0x4000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000];
const BMOVES_UL: [u64; 64] = [0x8040201008040200,0x0080402010080400,0x0000804020100800,0x0000008040201000,0x0000000080402000,0x0000000000804000,0x0000000000008000,0x0000000000000000,0x4020100804020000,0x8040201008040000,0x0080402010080000,0x0000804020100000,0x0000008040200000,0x0000000080400000,0x0000000000800000,0x0000000000000000,0x2010080402000000,0x4020100804000000,0x8040201008000000,0x0080402010000000,0x0000804020000000,0x0000008040000000,0x0000000080000000,0x0000000000000000,0x1008040200000000,0x2010080400000000,0x4020100800000000,0x8040201000000000,0x0080402000000000,0x0000804000000000,0x0000008000000000,0x0000000000000000,0x0804020000000000,0x1008040000000000,0x2010080000000000,0x4020100000000000,0x8040200000000000,0x0080400000000000,0x0000800000000000,0x0000000000000000,0x0402000000000000,0x0804000000000000,0x1008000000000000,0x2010000000000000,0x4020000000000000,0x8040000000000000,0x0080000000000000,0x0000000000000000,0x0200000000000000,0x0400000000000000,0x0800000000000000,0x1000000000000000,0x2000000000000000,0x4000000000000000,0x8000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000];
const BMOVES_DR: [u64; 64] = [0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000001,0x0000000000000002,0x0000000000000004,0x0000000000000008,0x0000000000000010,0x0000000000000020,0x0000000000000040,0x0000000000000000,0x0000000000000100,0x0000000000000201,0x0000000000000402,0x0000000000000804,0x0000000000001008,0x0000000000002010,0x0000000000004020,0x0000000000000000,0x0000000000010000,0x0000000000020100,0x0000000000040201,0x0000000000080402,0x0000000000100804,0x0000000000201008,0x0000000000402010,0x0000000000000000,0x0000000001000000,0x0000000002010000,0x0000000004020100,0x0000000008040201,0x0000000010080402,0x0000000020100804,0x0000000040201008,0x0000000000000000,0x0000000100000000,0x0000000201000000,0x0000000402010000,0x0000000804020100,0x0000001008040201,0x0000002010080402,0x0000004020100804,0x0000000000000000,0x0000010000000000,0x0000020100000000,0x0000040201000000,0x0000080402010000,0x0000100804020100,0x0000201008040201,0x0000402010080402,0x0000000000000000,0x0001000000000000,0x0002010000000000,0x0004020100000000,0x0008040201000000,0x0010080402010000,0x0020100804020100,0x0040201008040201];
const BMOVES_DL: [u64; 64] = [0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000000,0x0000000000000002,0x0000000000000004,0x0000000000000008,0x0000000000000010,0x0000000000000020,0x0000000000000040,0x0000000000000080,0x0000000000000000,0x0000000000000204,0x0000000000000408,0x0000000000000810,0x0000000000001020,0x0000000000002040,0x0000000000004080,0x0000000000008000,0x0000000000000000,0x0000000000020408,0x0000000000040810,0x0000000000081020,0x0000000000102040,0x0000000000204080,0x0000000000408000,0x0000000000800000,0x0000000000000000,0x0000000002040810,0x0000000004081020,0x0000000008102040,0x0000000010204080,0x0000000020408000,0x0000000040800000,0x0000000080000000,0x0000000000000000,0x0000000204081020,0x0000000408102040,0x0000000810204080,0x0000001020408000,0x0000002040800000,0x0000004080000000,0x0000008000000000,0x0000000000000000,0x0000020408102040,0x0000040810204080,0x0000081020408000,0x0000102040800000,0x0000204080000000,0x0000408000000000,0x0000800000000000,0x0000000000000000,0x0002040810204080,0x0004081020408000,0x0008102040800000,0x0010204080000000,0x0020408000000000,0x0040800000000000,0x0080000000000000,0x0000000000000000];

#[inline(always)]
pub fn bishop_bbmoves(square: usize) -> u64 {
    let main_bb: u64 = board::get_bitboard(12) | board::get_bitboard(13);

    let mut right: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) right,
            b = in(reg) BMOVES_UR[square]
        )
    }
    let mut left: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) left,
            b = in(reg) BMOVES_UL[square]
        )
    }

    let mut up: u64 = main_bb;
    // have to do this since bitintr pext takes like 10 clock cycles (ew)
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            a = inout(reg) up,
            b = in(reg) BMOVES_DR[square]
        )
    }

    // asm probably has a reverse bits command, but idk it so for now I'll just use bitintr since it is fast enough
    up = up.reverse_bits().blsmsk().reverse_bits();
    unsafe {
        asm!(
            "pdep {a}, {a}, {b}",
            a = inout(reg) up,
            b = in(reg) BMOVES_DR[square]
        )
    }


    let mut down: u64 = main_bb;
    // have to do this since bitintr pext takes like 10 clock cycles (ew)
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            a = inout(reg) down,
            b = in(reg) BMOVES_DL[square]
        )
    }

    // asm probably has a reverse bits command, but idk it so for now I'll just use bitintr since it is fast enough
    down = down.reverse_bits().blsmsk().reverse_bits();
    unsafe {
        asm!(
            "pdep {a}, {a}, {b}",
            a = inout(reg) down,
            b = in(reg) BMOVES_DL[square]
        )
    }

    return (right | left | up | down) & !board::get_bitboard(12 + board::color());
}

// Get all legal moves
pub fn pslegalmoves(moves: &mut Vec<(u64, u64, usize, usize)>) -> i32{
    //let mut moves: Vec<(u64, u64, usize)> = vec![];
    let mut mv_count: i32 = 0;

    // Get the pawn moves
    let mut pawns: u64 = board::get_bitboard(0 + board::color());
    let mut from: u64;
    let mut tos_ep: u64;
    let mut tos: u64;

    while pawns > 0 {
        from = pawns.blsi();

        // Get moveto bitboard
        if board::color() == 0 {
            tos = wpawn_bbmoves(from.trailing_zeros() as usize);
            tos_ep = wpawnep_bbmoves(from.trailing_zeros() as usize);
        } else {
            tos = bpawn_bbmoves(from.trailing_zeros() as usize);
            tos_ep = bpawnep_bbmoves(from.trailing_zeros() as usize);
        }
        
        // Handle promotions
        if tos & 0xFF000000000000FF > 0 {
            // Go through each "to" square and add to moves
            while tos > 0 {
                moves.push((from, tos.blsi(), 0 + board::color(), 3));
                moves.push((from, tos.blsi(), 0 + board::color(), 4));
                moves.push((from, tos.blsi(), 0 + board::color(), 5));
                moves.push((from, tos.blsi(), 0 + board::color(), 6));
                
                mv_count += 4;
                tos ^= tos.blsi();
            }
        } else {
            // Go through each "to" square and add to moves
            while tos > 0 {
                moves.push((from, tos.blsi(), 0 + board::color(), 0));
                
                mv_count += 1;
                tos ^= tos.blsi();
            }
            while tos_ep > 0 {
                moves.push((from, tos_ep.blsi(), 0 + board::color(), 1 + board::color()));

                mv_count += 1;
                tos_ep ^= tos_ep.blsi();
            }
        }
        pawns ^= pawns.blsi();
    }
    
    // Get the knight moves
    let mut knights: u64 = board::get_bitboard(2 + board::color());
    while knights > 0 {
        from = knights.blsi();
        

        tos = knight_bbmoves(from.trailing_zeros() as usize);

        while tos > 0 {
            moves.push((from, tos.blsi(), 2 + board::color(),0));
            mv_count += 1;
            tos ^= tos.blsi();
        }
        knights ^= knights.blsi();
    }

    // Get the bishop moves
    let mut bishops: u64 = board::get_bitboard(4 + board::color());
    while bishops > 0 {
        from = bishops.blsi();

        tos = bishop_bbmoves(from.trailing_zeros() as usize);

        while tos > 0 {
            moves.push((from, tos.blsi(), 4 + board::color(),0));
            mv_count += 1;
            tos ^= tos.blsi();
        }
        bishops ^= bishops.blsi();
    }

    // Get the rook moves
    let mut rooks: u64 = board::get_bitboard(6 + board::color());
    while rooks > 0 {
        from = rooks.blsi();
        tos = rook_bbmoves(from.trailing_zeros() as usize);

        while tos > 0 {
            moves.push((from, tos.blsi(), 6 + board::color(),0));
            mv_count += 1;
            tos ^= tos.blsi();
        }
        rooks ^= rooks.blsi();
    }

    // Get the queen moves
    let mut queens: u64 = board::get_bitboard(8 + board::color());
    while queens > 0 {
        from = queens.blsi();

        tos = queen_bbmoves(from.trailing_zeros() as usize);

        while tos > 0 {
            moves.push((from, tos.blsi(), 8 + board::color(),0));
            mv_count += 1;
            tos ^= tos.blsi();
        }
        queens ^= queens.blsi();
    }

    // Get the king moves
    let mut kings: u64 = board::get_bitboard(10 + board::color());
    while kings > 0 {
        from = kings.blsi();

        tos = king_bbmoves(from.trailing_zeros() as usize);
        while tos > 0 {
            moves.push((from, tos.blsi(), 10 + board::color(),0));
            mv_count += 1;
            tos ^= tos.blsi();
        }
        kings ^= kings.blsi();
    }
    // Castling

    
    if (board::get_bitboard(14) & 0b1000 > 0) & (( board::get_bitboard(12) | board::get_bitboard(13) ) & (0b0110) == 0) & (board::color() == 0) {
        // Make sure it isn't going through check
        let mut good: bool = true;
        if board::in_check() {
            good = false;
        }
        board::movebb(8, 4, 10, 0);
        board::change_turn();
        if board::in_check() {
            good = false;
        }
        board::change_turn();
        board::undo();
        if good {
            moves.push((8, 2, 10, 7));
            mv_count += 1;
        }
    }


    if (board::get_bitboard(14) & 0b0100 > 0) & (( board::get_bitboard(12) | board::get_bitboard(13) ) & (0b01110000) == 0) & (board::color() == 0) {

        // Make sure it isn't going through check
        let mut good: bool = true;
        if board::in_check() {
            good = false;
        }
        board::movebb(8, 16, 10, 0);
        board::change_turn();
        if board::in_check() {
            good = false;
        }
        board::movebb(16, 32, 10, 0);
        board::change_turn();
        if board::in_check() {
            good = false;
        }
        // Don't need these since two change turns cancel out
        // board::change_turn();
        // board::change_turn();
        board::undo();
        board::undo();
        if good {
            moves.push((8,32,10,8));
            mv_count += 1;
        }
    }


    if (board::get_bitboard(14) & 0b0010 > 0) & (( board::get_bitboard(12) | board::get_bitboard(13) ) & (0x600000000000000) == 0) & (board::color() == 1) {

        // Make sure it isn't going through check
        let mut good: bool = true;

        if board::in_check() {
            good = false;
        }
        board::movebb(0x800000000000000, 0x400000000000000, 11, 0);

        board::change_turn();
        if board::in_check() {
            
            good = false;
        }
        board::change_turn();
        board::undo();
        if good {
            moves.push((0x800000000000000, 0x200000000000000, 11, 9));
            mv_count += 1;
        }
    }

    if (board::get_bitboard(14) & 0b0001 > 0) & (( board::get_bitboard(12) | board::get_bitboard(13) ) & (0x7000000000000000) == 0) & (board::color() == 1) {
        // Make sure it isn't going through check
        let mut good: bool = true;
        if board::in_check() {
            good = false;
        }
        board::movebb(0x800000000000000, 0x1000000000000000, 11, 0);
        board::change_turn();
        if board::in_check() {
            good = false;
        }
        board::movebb(0x1000000000000000, 0x2000000000000000, 11, 0);
        board::change_turn();
        if board::in_check() {
            good = false;
        }
        board::undo();
        board::undo();
        if good {
            moves.push((0x800000000000000, 0x2000000000000000, 11, 10));
            mv_count += 1;
        }
    }

    return mv_count;
}

pub fn checkmsk(from:usize, ptype:usize) {
    
}

// Get specific directions for sliding pieces, from checkmsk
#[inline(always)]
pub fn b_ul (square: usize, main_bb: u64) -> u64 {
    let mut mvs: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) mvs,
            b = in(reg) BMOVES_UL[square]
        )
    }
    mvs
}
pub fn b_ur (square: usize, main_bb: u64) -> u64 {
    let mut mvs: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "blsmsk {a}, {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) mvs,
            b = in(reg) BMOVES_UR[square]
        )
    }
    mvs
}
pub fn b_dr (square: usize, main_bb: u64) -> u64 {
    let mut mvs: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "bswap {a}",
            "blsmsk {a}, {a}",
            "bswap {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) mvs,
            b = in(reg) BMOVES_DR[square]
        )
    }
    mvs
}
pub fn b_dl (square: usize, main_bb: u64) -> u64 {
    let mut mvs: u64 = main_bb;
    unsafe {
        asm!(
            "pext {a}, {a}, {b}",
            "bswap {a}",
            "blsmsk {a}, {a}",
            "bswap {a}",
            "pdep {a}, {a}, {b}",
            a = inout(reg) mvs,
            b = in(reg) BMOVES_DL[square]
        )
    }
    mvs
}