// Unfinished rework of board to use objects instead of global variables
// And also to be less bloated and generally better
pub use bitintr::*;
pub use crate::board;
pub use crate::moves;
pub use crate::utils::*;

// Enums
use PieceType::*;
use Color::*;
use Flag::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum PieceType {
    #[default]
    WhitePawn=0,BlackPawn=1,
    WhiteKnight=2,BlackKnight=3,
    WhiteBishop=4,BlackBishop=5,
    WhiteRook=6,BlackRook=7,
    WhiteQueen=8,BlackQueen=9,
    WhiteKing=10,BlackKing=11,
    WhitePieces=12,BlackPieces=13,
    // Not piecetypes
    // but are stored in bitboards
    CastleRights=14,
    EnPassant=15,
}
impl PieceType {
    pub fn shiftedby(self, color:Color) -> PieceType {
        match color {
            White => {self},
            Black => {num_to_piece(self as usize + 1)}
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Color {
    #[default] 
    Black=0,
    White=1,
}

impl Color {
    pub fn swapped(&mut self) -> Color {
        match self {
            Black => {White},
            White => {Black}
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Flag {
    #[default] 
    NoFlag,
    WhiteEnPassant,
    BlackEnPassant,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    WhiteKingsideCastle,
    WhiteQueensideCastle,
    BlackKingsideCastle,
    BlackQueensideCastle,
}
// Move
#[derive(Default, Clone, Copy)]
pub struct Move {
    pub from: u64,
    pub to: u64,
    pub piece_type: PieceType,
    pub flag: Flag,
}
impl Move {
    pub fn null() -> Self {
        Self {from: 0, to: 0, piece_type: WhitePawn, flag:NoFlag}
    }
}

pub struct MoveList {
    pub moves: [Move; 300],
    pub pos: usize
}
impl MoveList {
    pub fn new() -> Self {
        Self {moves: [Move::null(); 300], pos: 0}
    }
    pub fn push(&mut self, mv: Move) {
        self.moves[self.pos] = mv;
        self.pos += 1;
    }
    pub fn clear(&mut self) {
        self.pos = 0;
    }
}

// BoardState is just the pieces on the board
pub struct BoardState {
    // Bitboards
    bitboards: [u64; 16],
    // 50 Move Rule Counter
    move_counter: u64,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            bitboards:[65280, 71776119061217280, 66, 4755801206503243776, 36, 2594073385365405696, 129, 9295429630892703744, 16, 1152921504606846976, 8, 576460752303423488,65535, 18446462598732840960,0b1111,0,],
            move_counter:0,
        }
    }
    pub fn clone(&mut self) -> Self {
        BoardState {
            bitboards: self.bitboards,
            move_counter: self.move_counter
        }
    }
}

// Board Keeps track of history
pub struct Board {
    side_to_move: Color,
    state: BoardState,
    // All past states of the board (used for 50 move rule mostly)
    history: Vec<BoardState>,
    // For movegen
    checkmask: u64,
    attacked: u64,
}

// Main board implementation
impl Board {
    // Instantiate board
    pub fn new() -> Self {
        Board {
            side_to_move: Color::White,
            state: BoardState::new(),
            history: vec![],
            checkmask: u64::MAX,
            attacked: 0,
        }
    }
    pub fn update_attacked(&mut self, update: u64) {
        self.attacked |= update;
    }
    pub fn reset_attacked(&mut self) {
        self.attacked = 0;
    }
    pub fn attacked(&mut self) -> u64 {
        self.attacked
    }

    pub fn set_bitboard(&mut self, piece_type: PieceType, new_bitboard: u64) {
        self.state.bitboards[piece_type as usize] = new_bitboard;
    }
    pub fn set_move_count(&mut self, new_count: u64) {
        self.state.move_counter = new_count;
    }

    pub fn update_checkmask(&mut self, update: u64) {
        self.checkmask &= update;
    }
    pub fn reset_checkmask(&mut self) {
        self.checkmask = u64::MAX;
    }
    pub fn checkmask(&mut self) -> u64 {
        self.checkmask
    }
    // Remove from an entire square
    // Return true if any changes were made
    pub fn clear_square(&mut self, square: u64) -> bool {
        let saved = self.state.bitboards;

        self.state.bitboards[WhitePawn as usize] &= !square;
        self.state.bitboards[BlackPawn as usize ] &= !square;
        self.state.bitboards[WhiteKnight as usize ] &= !square;
        self.state.bitboards[BlackKnight as usize ] &= !square;
        self.state.bitboards[WhiteBishop as usize ] &= !square;
        self.state.bitboards[BlackBishop as usize ] &= !square;
        self.state.bitboards[WhiteRook as usize ] &= !square;
        self.state.bitboards[BlackRook as usize ] &= !square;
        self.state.bitboards[WhiteQueen as usize ] &= !square;
        self.state.bitboards[BlackQueen as usize ] &= !square;
        self.state.bitboards[WhiteKing as usize ] &= !square;
        self.state.bitboards[BlackKing as usize ] &= !square;
        self.state.bitboards[WhitePieces as usize ] &= !square;
        self.state.bitboards[BlackPieces as usize ] &= !square;

        saved == self.state.bitboards
    }

    pub fn add_to_square(&mut self, square: usize, piece_type: PieceType) {
        self.state.bitboards[piece_type as usize] |= 1 << square;
        self.state.bitboards[piece_type as usize % 2 + 12] |= 1 << square;
    }

    // Make a move
    pub fn make_move(&mut self, mv: &Move) {
        self.history.push(self.state.clone());

        self.state.move_counter += 1;
        self.state.bitboards[EnPassant as usize ] = 0;

        if mv.piece_type as i32 <= 1 {
            if mv.from == (mv.to << 16){
                self.state.bitboards[EnPassant as usize] = mv.to << 8;
            } else if mv.to == (mv.from << 16)  {
                self.state.bitboards[EnPassant as usize] = mv.from << 8;
            }
            self.state.move_counter = 0;
        }


        match mv.flag {
            NoFlag => {
                // Reset move counter if attacking
                self.state.move_counter *= !self.clear_square(mv.to) as u64;

                self.state.bitboards[mv.piece_type as usize ] ^= mv.from | mv.to;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.from | mv.to;
            }
            WhiteEnPassant => {
                // Remove attacked pawn
                self.state.bitboards[BlackPawn as usize] &= !(mv.to >> 8);
                self.state.bitboards[BlackPieces as usize] &= !(mv.to >> 8);

                // Move moving pawn
                self.state.bitboards[WhitePawn as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhitePieces as usize] ^= mv.to | mv.from;
            }
            BlackEnPassant => {
                self.state.bitboards[WhitePawn as usize] &= !(mv.to << 8);
                self.state.bitboards[WhitePieces as usize] &= !(mv.to << 8);

                self.state.bitboards[BlackPawn as usize] ^= mv.to | mv.from;
                self.state.bitboards[BlackPieces as usize] ^= mv.to | mv.from;
            }
            KnightPromotion => {
                // For capture + promote
                self.clear_square(mv.to);
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteKnight.shiftedby(self.color()) as usize] |= mv.to;
            }
            BishopPromotion => {
                // For capture + promote
                self.clear_square(mv.to);
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteBishop.shiftedby(self.color()) as usize] |= mv.to;
            }
            RookPromotion => {
                // For capture + promote
                self.clear_square(mv.to);
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteRook.shiftedby(self.color()) as usize] |= mv.to;
            }
            QueenPromotion => {
                // For capture + promote
                self.clear_square(mv.to);
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteQueen.shiftedby(self.color()) as usize] |= mv.to;
            }
            WhiteKingsideCastle => {
                // Move King
                self.state.bitboards[WhiteKing as usize] ^= mv.from | mv.to;
                // Move Rook
                self.state.bitboards[WhiteRook as usize] ^= 5;
                // Set Color Bitboard
                self.state.bitboards[WhitePieces as usize] ^= mv.from | mv.to | 5;
            }
            WhiteQueensideCastle => {
                self.state.bitboards[WhiteKing as usize] ^= mv.from | mv.to;
                self.state.bitboards[WhiteRook as usize] ^= 144;
                self.state.bitboards[WhitePieces as usize] ^= mv.from | mv.to | 144;
            }
            BlackKingsideCastle => {
                self.state.bitboards[BlackKing as usize] ^= mv.from | mv.to;
                self.state.bitboards[BlackRook as usize] ^= 0x500000000000000;
                self.state.bitboards[BlackPieces as usize] ^= mv.from | mv.to | 0x500000000000000;
            }
            BlackQueensideCastle => {
                self.state.bitboards[BlackKing as usize] ^= mv.from | mv.to;
                self.state.bitboards[BlackRook as usize] ^= 0x9000000000000000;
                self.state.bitboards[BlackPieces as usize] ^= mv.from | mv.to | 0x9000000000000000;
            }
        }
        // Change color
        self.switch_color();

        // Change Castling Rights
        if (self.state.bitboards[WhiteRook as usize] & 1) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b0111;
        }
        if (self.state.bitboards[WhiteRook as usize] & 0b10000000) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b1011;
        }
        if (self.state.bitboards[BlackRook as usize] & 0b0000000100000000000000000000000000000000000000000000000000000000) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b1101;
        }
        if (self.state.bitboards[BlackRook as usize] & 0b1000000000000000000000000000000000000000000000000000000000000000) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b1110;
        }

        if (self.state.bitboards[WhiteKing as usize] & 0b1000) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b0011;
        }
        if (self.state.bitboards[BlackKing as usize] & 0b0000100000000000000000000000000000000000000000000000000000000000) == 0 {
            self.state.bitboards[CastleRights as usize] &= 0b1100;
        }
    }

    pub fn switch_color(&mut self) {
        self.side_to_move = self.side_to_move.swapped();
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.side_to_move = new_color;
    }

    pub fn undo(&mut self) {
        self.state = self.history.pop().unwrap();
        self.switch_color();
    }

    pub fn reset_hist(&mut self) {
        self.history.clear();
    }

    pub fn clear(&mut self) {
        self.state.bitboards = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        self.reset_hist();
    }

    pub fn get_bitboard(&self, piece_type: PieceType) -> u64 {
        self.state.bitboards[piece_type as usize]
    }

    pub fn color(&self) -> Color {
        self.side_to_move
    }

    pub fn chess_to_move(&self, mv: String) -> Move {
        let pt = self.piece_on_sq(chess_to_square(String::from(&mv[0..2])));
        let flg = self.move_flag(&mv);
    
        Move {from: 1 << chess_to_square(String::from(&mv[0..2])), to: 1 << chess_to_square(String::from(&mv[2..4])), piece_type: num_to_piece(pt), flag: num_to_flag(flg)}
    }

    pub fn move_to_chess(&self, mv: Move) -> String {
        format!("{}{}{}", bbsquare_to_chess(mv.from), bbsquare_to_chess(mv.to), flag_to_piece(mv.flag))
    }

    pub fn piece_on_sq(&self, square: usize) -> usize{
        for i in 0..12 {
            if 1_u64.wrapping_shl(square as u32) & self.get_bitboard(num_to_piece(i)) > 0 {
                return i;
            }
        }
        0
    }

    pub fn move_flag(&self, mv: &str) -> usize {
        let mut flag = 0;
    
        let from = chess_to_square(String::from(&mv[0..2]));
        let to = 1 << chess_to_square(String::from(&mv[2..4])) as u64;
    
        // en passant
        let pt = self.piece_on_sq(chess_to_square(String::from(&mv[0..2])));
        if pt == 0 {
            if self.wpawn_epmoves(from) & to > 0 {
                flag = 1;
            }
        }
        if pt == 1 {
            if self.bpawn_epmoves(from) & to > 0 {
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
}

// Some useful functions
const PIECE_TYPES:[PieceType; 16] = [
    WhitePawn,BlackPawn,
    WhiteKnight,BlackKnight,
    WhiteBishop,BlackBishop,
    WhiteRook,BlackRook,
    WhiteQueen,BlackQueen,
    WhiteKing,BlackKing,
    WhitePieces,BlackPieces,
    CastleRights,
    EnPassant
];
pub fn num_to_piece(num: usize) -> PieceType {
    PIECE_TYPES[num]
}
const FLAGS:[Flag; 11] = [
    NoFlag,
    WhiteEnPassant,
    BlackEnPassant,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    WhiteKingsideCastle,
    WhiteQueensideCastle,
    BlackKingsideCastle,
    BlackQueensideCastle,
];
pub fn num_to_flag(num: usize) -> Flag {
    FLAGS[num]
}

pub fn flag_to_piece(flag: Flag) -> String {
    match flag {
        KnightPromotion => "n".to_string(),
        BishopPromotion => "b".to_string(),
        RookPromotion => "r".to_string(),
        QueenPromotion => "q".to_string(),
        _ => "".to_string()
    }
}