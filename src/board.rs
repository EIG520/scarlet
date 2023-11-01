pub use bitintr::*;
pub use crate::moves;
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;


static mut BITBOARDS: [u64; 17] = [
    // Pawns
    65280, 71776119061217280, 
    // Knights
    66, 4755801206503243776, 
    // Bishops
    36, 2594073385365405696, 
    // Rooks
    129, 9295429630892703744, 
    // Queens
    16, 1152921504606846976, 
    // Kings
    8, 576460752303423488,
    // White / Black
    65535, 18446462598732840960,
    // Castling rights : 1000 means "white can kingside castle", 0100 means "white can queenside castle", 0010 means "black can kingside castle", and 0001 means "black can queenside castle"
    0b1111,
    // Number of halfmoves since last capture/tactical move (for 50 move rule)
    0,
    // En Passant square
    0,
];

// Previous positions (so PREV but not including halfmoves)
lazy_static! {
    pub static ref PREV_POSES: Mutex<HashSet<[u64; 16]>> = Mutex::new(HashSet::new());
}
static mut PREV: [[u64; 17]; 10000] = [[0; 17]; 10000];
static mut POS: usize = 2;

static mut COLOR: usize = 0;

// Set given square to a specific piece
#[inline(always)]
pub fn set_square(square: usize, piece: usize) {
    del_from_square(square);
    unsafe {
        BITBOARDS[piece] |= 1 << square;
        BITBOARDS[12 + (piece % 2)] |= 1 << square;
    }
}

// Remove current piece at square
#[inline(always)]
pub fn del_from_square(square: usize) {
    let mut i: usize = 0;

    while i < 14 {
        unsafe{
            BITBOARDS[i] &= u64::MAX - (1 << square);
        }
        i+=1;
    }
}
pub fn set_bitboard(index: usize, value: u64) {
    unsafe {BITBOARDS[index] = value}
}
// Delete by bitboard (slightly faster since no left shift)
#[inline(always)]
pub fn del_from_squarebb(squarebb: u64) -> bool {
    let saved = unsafe{BITBOARDS};
    let mut i: usize = 0;

    while i < 14 {
        unsafe {
            BITBOARDS[i] &= u64::MAX - squarebb;
        }
        i += 1;
    }
    saved == unsafe{BITBOARDS}
}

// Move
pub fn movebb(frombb: u64, tobb: u64, piece: usize, flag: usize) {
    unsafe {
        PREV[POS] = BITBOARDS;
        PREV_POSES.lock().unwrap().insert([BITBOARDS[0], BITBOARDS[1], BITBOARDS[2], BITBOARDS[3], BITBOARDS[4], BITBOARDS[5], BITBOARDS[6], BITBOARDS[7], BITBOARDS[8], BITBOARDS[9], BITBOARDS[10], BITBOARDS[11], BITBOARDS[12], BITBOARDS[13], BITBOARDS[14] /* Skip 15 */, BITBOARDS[16]]);
        POS += 1;
        BITBOARDS[15] += 1;
        if piece == 0 {
            BITBOARDS[15] = 0
        }

        BITBOARDS[16] = 0;

        match flag {
            // Regular Move
            0 => {
                // Clear other piece bitboards if it is an attacking move
                BITBOARDS[15] *= (!del_from_squarebb(tobb)) as u64;

                BITBOARDS[piece] ^= frombb | tobb;
                BITBOARDS[12 + color()] ^= frombb | tobb;

                // print_bb(frombb << 16);
                // print_bb(tobb);
                if (piece == 0) & (frombb << 16 == tobb) {
                    BITBOARDS[16] = tobb >> 8;
                } else if (piece == 1) & (frombb >> 16 == tobb) {
                    BITBOARDS[16] = tobb << 8;
                }
            }
            // En passant
            1 => {
                BITBOARDS[1] &= u64::MAX - (tobb >> 8);
                BITBOARDS[13] &= u64::MAX - (tobb >> 8);

                BITBOARDS[piece] ^= frombb | tobb;
                BITBOARDS[12 + color()] ^= frombb | tobb;
            }
            2 => {
                BITBOARDS[0] &= u64::MAX - (tobb << 8);
                BITBOARDS[12] &= u64::MAX - (tobb << 8);

                BITBOARDS[piece] ^= frombb | tobb;
                BITBOARDS[12 + color()] ^= frombb | tobb;
            }
            // Promotions
            3 => {
                // Remove pawn
                BITBOARDS[0 + color()] ^= frombb;
                // Remove other pieces (I.E. if capture and promote)
                del_from_squarebb(tobb);
                // Change global bitboard
                BITBOARDS[12 + color()] ^= frombb | tobb;
                // Add knight
                BITBOARDS[2 + color()] ^= tobb;
            }
            4 => {
                // Remove pawn
                BITBOARDS[0 + color()] ^= frombb;
                // Remove other pieces (I.E. if capture and promote)
                del_from_squarebb(tobb);
                // Change global bitboard
                BITBOARDS[12 + color()] ^= frombb | tobb;
                // Add bishop
                BITBOARDS[4 + color()] ^= tobb;
            }
            5 => {
                // Remove pawn
                BITBOARDS[0 + color()] ^= frombb;
                // Remove other pieces (I.E. if capture and promote)
                del_from_squarebb(tobb);
                // Change global bitboard
                BITBOARDS[12 + color()] ^= frombb | tobb;
                // Add rook
                BITBOARDS[6 + color()] ^= tobb;
            }
            6 => {
                // Remove pawn
                BITBOARDS[0 + color()] ^= frombb;
                // Remove other pieces (I.E. if capture and promote)
                del_from_squarebb(tobb);
                // Change global bitboard
                BITBOARDS[12 + color()] ^= frombb | tobb;
                // Add queen
                BITBOARDS[8 + color()] ^= tobb;
            }
            // White castle kingside
            7 => {
                // Move king
                BITBOARDS[10] ^= frombb | tobb;
                // Move rook
                BITBOARDS[6] ^= 5;
                // Set color bitboard
                BITBOARDS[12] ^= frombb | tobb | 5;
            }
            // White castle queenside
            8 => {
                // Move king
                BITBOARDS[10] ^= frombb | tobb;
                // Move rook
                BITBOARDS[6] ^= 144;
                // Set color bitboard
                BITBOARDS[12] ^= frombb | tobb | 144;
            }
            // Black castle kingside
            9 => {
                // Move king
                BITBOARDS[11] ^= frombb | tobb;
                // Move rook
                BITBOARDS[7] ^= 0x500000000000000;
                // Set color bitboard
                BITBOARDS[13] ^= frombb | tobb | 0x500000000000000;
            }
            // Black castle queenside
            10 => {
                // Move king
                BITBOARDS[11] ^= frombb | tobb;
                // Move rook
                BITBOARDS[7] ^= 0x9000000000000000;
                // Set color bitboard
                BITBOARDS[13] ^= frombb | tobb | 0x9000000000000000;
            }
            _ => {}
        }
        COLOR = (COLOR + 1) % 2;

        if (BITBOARDS[6] & 1) == 0{
            BITBOARDS[14] &= 0b0111;
        }
        if (BITBOARDS[6] & 0b10000000) == 0 {
            BITBOARDS[14] &= 0b1011;
        }
        if (BITBOARDS[7] & 0b0000000100000000000000000000000000000000000000000000000000000000) == 0 {
            BITBOARDS[14] &= 0b1101
        }
        if (BITBOARDS[7] & 0b1000000000000000000000000000000000000000000000000000000000000000) == 0 {
            BITBOARDS[14] &= 0b1110;
        }
        if (BITBOARDS[10] & 0b1000) == 0 {
            BITBOARDS[14] &= 0b0011;
        }
        if (BITBOARDS[11] & 0b0000100000000000000000000000000000000000000000000000000000000000) == 0 {
            BITBOARDS[14] &= 0b1100;
        }
    }
}
// Detect game endings
pub fn is_repetition() -> bool {
    unsafe{PREV_POSES.lock().unwrap().contains(&[BITBOARDS[0], BITBOARDS[1], BITBOARDS[2], BITBOARDS[3], BITBOARDS[4], BITBOARDS[5], BITBOARDS[6], BITBOARDS[7], BITBOARDS[8], BITBOARDS[9], BITBOARDS[10], BITBOARDS[11], BITBOARDS[12], BITBOARDS[13], BITBOARDS[14] /* Skip 15 */, BITBOARDS[16]])}
}
pub fn is_50mr() -> bool {
    unsafe{BITBOARDS[15] >= 100}
}

pub fn state() -> i32 {
    if is_repetition() | is_50mr() {
        return 1;
    }
    if moves::legalmoves(&mut [(0 as u64,0 as u64,0 as usize,0 as usize);300]) == 0 {
        if in_check() {return 2;}
        return 1;
    }
    0
}

// Undo Move
#[inline(always)]
pub fn undo() {
    unsafe {
        POS -= 1;
        BITBOARDS = PREV[POS];
        PREV_POSES.lock().unwrap().remove(&[BITBOARDS[0], BITBOARDS[1], BITBOARDS[2], BITBOARDS[3], BITBOARDS[4], BITBOARDS[5], BITBOARDS[6], BITBOARDS[7], BITBOARDS[8], BITBOARDS[9], BITBOARDS[10], BITBOARDS[11], BITBOARDS[12], BITBOARDS[13], BITBOARDS[14] /* Skip 15 */, BITBOARDS[16]]);
        
        //BITBOARDS[15] -= 1;
        COLOR = (COLOR + 1) % 2;
    }
    
}

pub fn reset_hist() {
    unsafe {
        PREV = [[0;17];10000];
        PREV_POSES.lock().unwrap().clear();
        POS = 2;
    }
}

// Get piece bitboard

#[inline(always)]
pub fn get_bitboard(piece: usize) -> u64 {
    return unsafe { BITBOARDS[piece] }
}
#[inline(always)]
pub fn get_prev_bitboard(piece: usize) -> u64 {
    return unsafe { PREV[POS - 1][piece] };
}

// I know I should be making color a bool or at least checking that bool < 2 but shut up
pub fn index_with_color(piece: usize, color: usize) -> usize {
    return 2*piece + color;
}

// Convert a place in the bitboard to its xy coords
pub fn square_to_xy(square: usize) -> (usize, usize) {
    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    return (x,y);
}

// Convert a place in the bitboard to chess (e.g. a4)
pub fn square_to_chess(square: usize) -> String {
    let pos_to_char: [&str; 8] = ["h", "g", "f", "e", "d", "c", "b", "a"];

    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    return String::from(pos_to_char[x]) + &format!("{}", y + 1);
}

// something like ..001000 to 4
pub fn bbsquare_to_chess(square: u64) -> String {
    return square_to_chess(square.trailing_zeros() as usize);
}

// my silly move rep into a2a4 or whatever
pub fn move_to_chess(mv: (u64, u64, usize,usize)) -> String {
    return format!("{}{}{}",bbsquare_to_chess(mv.0),bbsquare_to_chess(mv.1),flag_to_piece(mv.3));
}
pub fn flag_to_piece(flag: usize) -> String {
    match flag {
        3 => "n".to_string(),
        4 => "b".to_string(),
        5 => "r".to_string(),
        6 => "q".to_string(),
        _ => "".to_string()
    }
}

// Spaghetti (yum)
pub fn chesscol_to_square(col: char) -> usize{
    if col == 'a' {
        return 7;
    }
    if col == 'b' {
        return 6;
    }
    if col == 'c' {
        return 5;
    }
    if col == 'd' {
        return 4;
    }
    if col == 'e' {
        return 3;
    }
    if col == 'f' {
        return 2;
    }
    if col == 'g' {
        return 1;
    }
    if col == 'h' {
        return 0;
    }
    // return number >64 if invalid
    return 65;
}

#[inline(always)]
pub fn chess_to_square(square: String) -> usize {
    if square.chars().collect::<Vec<char>>().len() < 2 {return 65;}

    let x: char  = square.chars().collect::<Vec<char>>()[1];
    let y: usize = chesscol_to_square(square.chars().next().unwrap());

    if x.is_digit(10) {
        return 8*(x.to_digit(10).unwrap() as usize - 1) + y;
    } else {
        // Return number >64 if invalid
        return 65;
    }
}

#[inline(always)]
pub fn chess_to_move(mv: String) -> (u64, u64, usize, usize){
    let pt = piece_on_sq(chess_to_square(String::from(&mv[0..2])));
    let flag = move_flag(&mv);

    return (1 << chess_to_square(String::from(&mv[0..2])), 1 << chess_to_square(String::from(&mv[2..4])), pt, flag);
}

#[inline(always)]
pub fn color() -> usize {
    return unsafe { COLOR };
}

pub fn set_color(color: usize) {
    unsafe {COLOR = color}
}
#[inline(always)]
pub fn change_turn() {
    unsafe {COLOR = (COLOR + 1) % 2;}
}

// Detect Checks

// pawns :(
const ATK_WPMOVES: [u64; 64] = [0x0000000000000200, 0x0000000000000500, 0x0000000000000a00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000a000, 0x0000000000004000, 0x0000000000020000, 0x0000000000050000, 0x00000000000a0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000a00000, 0x0000000000400000, 0x0000000002000000, 0x0000000005000000, 0x000000000a000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000a0000000, 0x0000000040000000, 0x0000000200000000, 0x0000000500000000, 0x0000000a00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000a000000000, 0x0000004000000000, 0x0000020000000000, 0x0000050000000000, 0x00000a0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000a00000000000, 0x0000400000000000, 0x0002000000000000, 0x0005000000000000, 0x000a000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00a0000000000000, 0x0040000000000000, 0x0200000000000000, 0x0500000000000000, 0x0a00000000000000, 0x1400000000000000, 0x2800000000000000, 0x5000000000000000, 0xa000000000000000, 0x4000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000];
const ATK_BPMOVES: [u64; 64] = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000002, 0x0000000000000005, 0x000000000000000a, 0x0000000000000014, 0x0000000000000028, 0x0000000000000050, 0x00000000000000a0, 0x0000000000000040, 0x0000000000000200, 0x0000000000000500, 0x0000000000000a00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000a000, 0x0000000000004000, 0x0000000000020000, 0x0000000000050000, 0x00000000000a0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000a00000, 0x0000000000400000, 0x0000000002000000, 0x0000000005000000, 0x000000000a000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000a0000000, 0x0000000040000000, 0x0000000200000000, 0x0000000500000000, 0x0000000a00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000a000000000, 0x0000004000000000, 0x0000020000000000, 0x0000050000000000, 0x00000a0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000a00000000000, 0x0000400000000000, 0x0002000000000000, 0x0005000000000000, 0x000a000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00a0000000000000, 0x0040000000000000];


pub fn in_check() -> bool {
    let pos: usize = get_bitboard(10 + color()).trailing_zeros() as usize;
    let acol: usize = (color() + 1) % 2;


    // knight checks
    return (moves::knight_bbmoves(pos) & get_bitboard(2 + acol) > 0)
    // bishop checks (and queen)
    | (moves::bishop_bbmoves(pos) & (get_bitboard(4 + acol) | get_bitboard(8 + acol)) > 0)
    // rook checks (and queen)
    | ((moves::rook_bbmoves(pos) & (get_bitboard(6 + acol) | get_bitboard(8 + acol))) > 0)
    // white pawn checks
    | (((ATK_BPMOVES[pos] & get_bitboard(0)) > 0) & (acol == 0))
    // black pawn checks
    | (((ATK_WPMOVES[pos] & get_bitboard(1)) > 0) & (acol == 1))
    // king checks
    | (moves::king_bbmoves(pos) & get_bitboard(10 + acol) > 0);
}

// Print out a bitboard as 8x8
pub fn print_bb(bitboard: u64) {
    let bb: String = format!("{:064b}", bitboard);

    let mut i = 0;
    for ch in bb.chars() {
        print!("{}", ch);
        i+=1;
        if i == 8 {
            i = 0;
            print!("\n");
        }
    }
    println!();
}

pub fn piece_on_sq(square: usize) -> usize{
    for i in 0..12 {
        if 1_u64.wrapping_shl(square as u32) & get_bitboard(i) > 0 {
            return i;
        }
    }
    0
}

pub fn move_flag(mv: &str) -> usize {
    let mut flag = 0;

    let from = chess_to_square(String::from(&mv[0..2]));
    let to = 1 << chess_to_square(String::from(&mv[2..4])) as u64;

    // en passant
    let pt = piece_on_sq(chess_to_square(String::from(&mv[0..2])));
    if pt == 0 {
        if moves::wpawnep_bbmoves(from) & to > 0 {
            flag = 1;
        }
    }
    if pt == 1 {
        if moves::bpawnep_bbmoves(from) & to > 0 {
            flag = 2;
        }
    }

    // Promotions
    if mv.len() == 5 {
        match mv.chars().nth(4).unwrap() {
            'n' => {flag = 3},
            'b' => {flag = 4},
            'r' => {flag = 5},
            'q' => {flag = 6},
            _ => {}
        }
    }

    // Castling
    if pt == 10 || pt == 11 {
        match mv {
            "e1g1" => {flag = 7},
            "e1c1" => {flag = 8},
            "e8g8" => {flag = 9},
            "e8c8" => {flag = 10},
            _ => {}
        }
    }
    flag
}