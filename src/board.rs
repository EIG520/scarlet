pub use bitintr::*;
pub use crate::moves;


static mut BITBOARDS: [u64; 16] = [
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
    // Black / White
    65535, 18446462598732840960,
    // For an optimization later
    0,0
];

static mut PREV: [[u64; 16]; 10000] = [[0; 16]; 10000];
static mut POS: usize = 2;

static mut COLOR: usize = 0;

// Set given square to a specific piece
pub fn set_square(square: usize, piece: usize) {
    del_from_square(square);
    unsafe {
        BITBOARDS[piece] |= 1 << square;
        BITBOARDS[12 + (piece % 2)] |= 1 << square;
    }
}

// Remove current piece at square
pub fn del_from_square(square: usize) {
    let mut i: usize = 0;

    while i < 14 {
        unsafe{
            BITBOARDS[i] &= u64::MAX - 1 << square;
        }
        i+=1;
    }
}
// Delete by bitboard (slightly faster since no left shift)
pub fn del_from_squarebb(squarebb: u64) {
    let mut i: usize = 0;

    while i < 14 {
        unsafe {
            BITBOARDS[i] &= u64::MAX - squarebb;
        }
        i += 1;
    }
}

// Move
pub fn movebb(frombb: u64, tobb: u64, piece: usize) {
    unsafe {
        PREV[POS] = BITBOARDS;
        POS += 1;

        if (piece == 0) & !((tobb == frombb << 8) | (tobb == frombb << 16)) & (get_bitboard(13) & tobb == 0) {
            BITBOARDS[1] &= u64::MAX - (tobb >> 8);
            BITBOARDS[13] &= u64::MAX - (tobb >> 8);
        } else if (piece == 1) & !((tobb == frombb >> 8) | (tobb == frombb >> 16)) & (get_bitboard(12) & tobb == 0) {
            BITBOARDS[0] &= u64::MAX - (tobb << 8);
            BITBOARDS[12] &= u64::MAX - (tobb >> 8);
        }

        del_from_squarebb(tobb);

        BITBOARDS[piece] ^= frombb | tobb;
        BITBOARDS[12 + color()] ^= frombb | tobb;

        COLOR = (COLOR + 1) % 2;
    }
}

// Undo Move
pub fn undo() {
    unsafe {
        POS -= 1;
        BITBOARDS = PREV[POS];
        
        COLOR = (COLOR + 1) % 2;
    }
}

// Get piece bitboard
pub fn get_bitboard(piece: usize) -> u64 {
    return unsafe { BITBOARDS[piece] }
}
pub fn get_prev_bitboard(piece: usize) -> u64 {
    return unsafe { PREV[POS - 2][piece] };
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
pub fn move_to_chess(mv: (u64, u64, usize)) -> String {
    return format!("{}{}",bbsquare_to_chess(mv.0),bbsquare_to_chess(mv.1));
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
    return 0;
}

pub fn chess_to_square(square: String) -> usize {
    let x: usize = square.chars().collect::<Vec<char>>()[1].to_digit(10).unwrap() as usize - 1;
    let y: usize = chesscol_to_square(square.chars().next().unwrap());

    return 8*x + y;
}

pub fn chess_to_move(mv: String, pt: usize) -> (u64, u64, usize){
    return (1 << chess_to_square(String::from(&mv[0..2])), 1 << chess_to_square(String::from(&mv[2..4])), pt);
}

pub fn color() -> usize {
    return unsafe { COLOR };
}

pub fn change_turn() {
    unsafe {COLOR = (COLOR + 1) % 2;}
}

// Detect Checks

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
    | (((( 1 << (pos - 9) | 1 << (pos - 7)) & get_bitboard(0)) > 0) & (acol == 0))
    // black pawn checks
    | (((( 1 << (pos + 9) | 1 << (pos + 7)) & get_bitboard(1)) > 0) & (acol == 1));
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
            println!();
        }
    }
    println!();
}