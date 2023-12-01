// FINISHED rework of board to use objects instead of global variables
// And also to be less bloated and generally better
pub use bitintr::*;
pub use fxhash::FxHashSet;

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

#[derive(Default, Clone, Copy, PartialEq)]
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
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: u64,
    pub to: u64,
    pub piece_type: PieceType,
    pub flag: Flag,
}
impl Move {
    pub const fn null() -> Self {
        Self {from: 0, to: 0, piece_type: WhitePawn, flag:NoFlag}
    }
}
pub struct MoveList {
    pub moves: Vec<Move>,
    pub pos: usize
}
impl MoveList {
    pub fn default() -> Self {
        Self {moves: Vec::with_capacity(100), pos: 0}
    }
    pub fn new(moves: Vec<Move>, len: usize) -> Self {
        Self {moves: moves, pos: len}
    }
    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv);
        self.pos += 1;
    }
    pub fn clear(&mut self) {
        self.moves.clear();
        self.pos = 0;
    }
}

// BoardState is just the pieces on the board
#[derive(Copy, Clone)]
pub struct BoardState {
    // Bitboards
    bitboards: [u64; 16],
    zobrist_hash: u64,
    mg_eval: i32,
    eg_eval: i32,
    phase: i32,
    // 50 Move Rule Counter (unused)
    move_counter: u64,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            bitboards:[65280, 71776119061217280, 66, 4755801206503243776, 36, 2594073385365405696, 129, 9295429630892703744, 16, 1152921504606846976, 8, 576460752303423488,65535, 18446462598732840960,0b1111,0,],
            zobrist_hash:0,
            mg_eval: 0,
            eg_eval: 0,
            phase: 0,
            move_counter:0,
        }
    }
    pub fn clone(&mut self) -> Self {
        BoardState {
            bitboards: self.bitboards,
            zobrist_hash: self.zobrist_hash,
            mg_eval: self.mg_eval,
            eg_eval: self.eg_eval,
            phase: self.phase,
            move_counter: self.move_counter
        }
    }
    pub fn bitboards(&self) -> [u64; 16] {
        self.bitboards
    }
}

// Keeps track of repetitions
pub struct RepetitionTracker {
    hashset1: FxHashSet<u64>,
    hashset2: FxHashSet<u64>,
    is_draw: bool,
}
impl RepetitionTracker {
    pub fn add(&mut self, key: u64) {
        match (self.hashset1.contains(&key), self.hashset2.contains(&key)) {
            (true, true) => {self.is_draw = true;},
            (true, false) => {self.hashset2.insert(key);},
            (false, true) => {},
            (false, false) => {self.hashset1.insert(key);},
        }
    }
    pub fn remove(&mut self, key: u64) {
        match (self.hashset1.contains(&key), self.hashset2.contains(&key)) {
            (true, true) => {self.is_draw = false;self.hashset2.remove(&key);},
            (true, false) => {self.hashset1.remove(&key);},
            (false, true) => {},
            (false, false) => {},
        }  
    }
    pub fn reset(&mut self) {
        self.hashset1.clear();
        self.hashset2.clear();
        self.is_draw = false;
    }
    pub fn new() -> Self {
        RepetitionTracker { hashset1: FxHashSet::default(), hashset2: FxHashSet::default(), is_draw: false }
    }
    pub fn is_repetition(&self) -> bool {
        self.is_draw
    }
}

// Board Keeps track of history
pub struct Board {
    side_to_move: Color,
    state: BoardState,
    // All past states of the board
    history: Vec<BoardState>,
    repetition_tracker: RepetitionTracker,
    // For movegen
    checkmask: u64,
    attacked: u64,
}

// Main board implementation
impl Board {
    // Instantiate board
    pub fn new() -> Self {
        let mut cur = Board {
            side_to_move: Color::White,
            state: BoardState::new(),
            history: vec![],
            repetition_tracker: RepetitionTracker::new(),
            checkmask: u64::MAX,
            attacked: 0,
        };
        cur.init();
        cur
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
    pub fn state(&self) -> &BoardState {
        &self.state
    }
    pub fn zobrist_hash(&self) -> u64 {
        self.state.zobrist_hash
    }
    pub fn set_zobrist_hash(&mut self, new_hash: u64) {
        self.state.zobrist_hash = new_hash;
    }
    pub fn set_mg_eval(&mut self, new_eval: i32) {
        self.state.mg_eval = new_eval;
    }
    pub fn set_eg_eval(&mut self, new_eval: i32) {
        self.state.eg_eval = new_eval;
    }
    pub fn set_phase(&mut self, new_phase: i32) {
        self.state.phase = new_phase;
    }
    pub fn mg_eval(&self) -> i32 {
        self.state.mg_eval
    }
    pub fn eg_eval(&self) -> i32 {
        self.state.eg_eval
    }
    pub fn phase(&self) -> i32 {
        self.state.phase
    }
    pub fn eval(&mut self) -> i32 {
        -1 * (self.state.mg_eval * self.state.phase + self.state.eg_eval * (30 - self.state.phase)) / 30 * (self.color() as i32 * -2 + 1)
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
    pub fn repinfo(&self) {
        println!("{:?}", self.repetition_tracker.hashset1);
    }
    // Remove from an entire square
    // Return true if any changes were made
    pub fn clear_square(&mut self, square: u64) -> bool {
        let saved = self.state.bitboards;

        self.update_zobrist_hash_square(square);
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
        self.update_zobrist_hash_square(square);

        saved == self.state.bitboards
    }

    pub fn add_to_square(&mut self, square: usize, piece_type: PieceType) {
        self.state.bitboards[piece_type as usize] |= 1 << square;
        self.state.bitboards[piece_type as usize % 2 + 12] |= 1 << square;
    }

    pub fn is_repetition(&self) -> bool {
        self.repetition_tracker.is_repetition()
    }

    // Make a move
    pub fn make_move(&mut self, mv: &Move) {
        self.history.push(self.state.clone());

        self.state.move_counter += 1;

        self.update_zobrist_hash_en_passant();
        self.state.bitboards[EnPassant as usize] = 0;

        if mv.piece_type as i32 <= 1 {
            if mv.from == (mv.to << 16){
                self.state.bitboards[EnPassant as usize] = mv.to << 8;
            } else if mv.to == (mv.from << 16)  {
                self.state.bitboards[EnPassant as usize] = mv.from << 8;
            }
            self.state.move_counter = 0;
        }
        self.update_zobrist_hash_en_passant();

        // self.evaluate();
        // println!("{}", self.mg_eval());

        match mv.flag {
            NoFlag => {
                // Reset move counter if attacking
                if mv.to & self.get_bitboard(WhitePieces.shiftedby(self.color().swapped())) > 0{
                    self.update_eval_capture(num_to_piece(self.piece_on_sq(mv.to.trailing_zeros() as usize)), mv.to);
                    self.state.move_counter = 0;
                    self.clear_square(mv.to);
                }

                self.state.bitboards[mv.piece_type as usize ] ^= mv.from | mv.to;
                self.update_zobrist_hash(mv.from, mv.piece_type);
                self.update_zobrist_hash(mv.to, mv.piece_type);
                self.update_eval(mv.piece_type, mv.from, mv.to);

                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.from | mv.to;
            }
            WhiteEnPassant => {
                // Remove attacked pawn
                self.state.bitboards[BlackPawn as usize] ^= mv.to >> 8;
                self.state.bitboards[BlackPieces as usize] ^= mv.to >> 8;
                self.update_zobrist_hash(mv.to >> 8, BlackPawn);
                self.update_eval_capture(BlackPawn, mv.to >> 8);

                // Move moving pawn
                self.state.bitboards[WhitePawn as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhitePieces as usize] ^= mv.to | mv.from;
                self.update_zobrist_hash(mv.to, WhitePawn);
                self.update_zobrist_hash(mv.from, WhitePawn);
                self.update_eval(mv.piece_type, mv.from, mv.to);
            }
            BlackEnPassant => {
                self.state.bitboards[WhitePawn as usize] ^= mv.to << 8;
                self.state.bitboards[WhitePieces as usize] ^= mv.to << 8;
                self.update_zobrist_hash(mv.to << 8, WhitePawn);
                self.update_eval_capture(WhitePawn, mv.to << 8);

                self.state.bitboards[BlackPawn as usize] ^= mv.to | mv.from;
                self.state.bitboards[BlackPieces as usize] ^= mv.to | mv.from;
                self.update_zobrist_hash(mv.to, BlackPawn);
                self.update_zobrist_hash(mv.from, BlackPawn);
                self.update_eval(mv.piece_type, mv.from, mv.to);
            }
            KnightPromotion => {
                // For capture + promote
                if mv.to & self.get_bitboard(WhitePieces.shiftedby(self.color().swapped())) > 0{
                    self.update_eval_capture(num_to_piece(self.piece_on_sq(mv.to.trailing_zeros() as usize)), mv.to);
                    self.state.move_counter = 0;
                    self.clear_square(mv.to);
                }
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteKnight.shiftedby(self.color()) as usize] ^= mv.to;
                self.update_zobrist_hash(mv.from, mv.piece_type);
                self.update_zobrist_hash(mv.to, WhiteKnight.shiftedby(self.color()));
                self.update_eval_promotion(WhiteKnight.shiftedby(self.color()), mv.from, mv.to);
            }
            BishopPromotion => {
                // For capture + promote
                if mv.to & self.get_bitboard(WhitePieces.shiftedby(self.color().swapped())) > 0{
                    self.update_eval_capture(num_to_piece(self.piece_on_sq(mv.to.trailing_zeros() as usize)), mv.to);
                    self.state.move_counter = 0;
                    self.clear_square(mv.to);
                }
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.state.bitboards[WhiteBishop.shiftedby(self.color()) as usize] |= mv.to;
                self.update_zobrist_hash(mv.from, mv.piece_type);
                self.update_zobrist_hash(mv.to, WhiteBishop.shiftedby(self.color()));
                self.update_eval_promotion(WhiteBishop.shiftedby(self.color()), mv.from, mv.to);
            }
            RookPromotion => {
                // For capture + promote
                if mv.to & self.get_bitboard(WhitePieces.shiftedby(self.color().swapped())) > 0{
                    self.update_eval_capture(num_to_piece(self.piece_on_sq(mv.to.trailing_zeros() as usize)), mv.to);
                    self.state.move_counter = 0;
                    self.clear_square(mv.to);
                }
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhiteRook.shiftedby(self.color()) as usize] |= mv.to;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.update_zobrist_hash(mv.from, mv.piece_type);
                self.update_zobrist_hash(mv.to, WhiteRook.shiftedby(self.color()));
                self.update_eval_promotion(WhiteRook.shiftedby(self.color()), mv.from, mv.to);
            }
            QueenPromotion => {
                // For capture + promote
                if mv.to & self.get_bitboard(WhitePieces.shiftedby(self.color().swapped())) > 0{
                    self.update_eval_capture(num_to_piece(self.piece_on_sq(mv.to.trailing_zeros() as usize)), mv.to);
                    self.state.move_counter = 0;
                    self.clear_square(mv.to);
                }
                // Piece changes
                self.state.bitboards[mv.piece_type as usize] ^= mv.from;
                self.state.bitboards[WhiteQueen.shiftedby(self.color()) as usize] |= mv.to;
                self.state.bitboards[WhitePieces.shiftedby(self.color()) as usize] ^= mv.to | mv.from;
                self.update_zobrist_hash(mv.from, mv.piece_type);
                self.update_zobrist_hash(mv.to, WhiteQueen.shiftedby(self.color()));
                self.update_eval_promotion(WhiteQueen.shiftedby(self.color()), mv.from, mv.to);
            }
            WhiteKingsideCastle => {
                // Move King
                self.state.bitboards[WhiteKing as usize] ^= mv.from | mv.to;
                self.update_zobrist_hash(mv.from, WhiteKing);
                self.update_zobrist_hash(mv.to, WhiteKing);
                self.update_eval(WhiteKing, mv.from, mv.to);

                // Move Rook
                self.state.bitboards[WhiteRook as usize] ^= 5;
                self.update_zobrist_hash(0b1, WhiteRook);
                self.update_zobrist_hash(0b100, WhiteRook);
                self.update_eval(WhiteRook, 0b1, 0b100);

                // Set Color Bitboard
                self.state.bitboards[WhitePieces as usize] ^= mv.from | mv.to | 5;                
            }
            WhiteQueensideCastle => {
                self.state.bitboards[WhiteKing as usize] ^= mv.from | mv.to;
                self.update_zobrist_hash(mv.from, WhiteKing);
                self.update_zobrist_hash(mv.to, WhiteKing);
                self.update_eval(WhiteKing, mv.from, mv.to);

                self.state.bitboards[WhiteRook as usize] ^= 144;
                self.update_zobrist_hash(0b10000, WhiteRook);
                self.update_zobrist_hash(0b10000000, WhiteRook);
                self.update_eval(WhiteRook, 0b10000000, 0b10000);

                self.state.bitboards[WhitePieces as usize] ^= mv.from | mv.to | 144;
            }
            BlackKingsideCastle => {
                self.state.bitboards[BlackKing as usize] ^= mv.from | mv.to;
                self.update_zobrist_hash(mv.from, BlackKing);
                self.update_zobrist_hash(mv.to, BlackKing);
                self.update_eval(BlackKing, mv.from, mv.to);

                self.state.bitboards[BlackRook as usize] ^= 0x500000000000000;
                self.update_zobrist_hash(0b100000000000000000000000000000000000000000000000000000000, BlackRook);
                self.update_zobrist_hash(0b10000000000000000000000000000000000000000000000000000000000, BlackRook);
                self.update_eval(BlackRook, 0b100000000000000000000000000000000000000000000000000000000, 0b10000000000000000000000000000000000000000000000000000000000);
                self.state.bitboards[BlackPieces as usize] ^= mv.from | mv.to | 0x500000000000000;
            }
            BlackQueensideCastle => {
                self.state.bitboards[BlackKing as usize] ^= mv.from | mv.to;
                self.update_zobrist_hash(mv.from, BlackKing);
                self.update_zobrist_hash(mv.to, BlackKing);
                self.update_eval(BlackKing, mv.from, mv.to);
                self.state.bitboards[BlackRook as usize] ^= 0x9000000000000000;
                self.update_zobrist_hash(0b1000000000000000000000000000000000000000000000000000000000000, BlackRook);
                self.update_zobrist_hash(0b1000000000000000000000000000000000000000000000000000000000000000, BlackRook);
                self.update_eval(BlackRook, 0b1000000000000000000000000000000000000000000000000000000000000000, 0b1000000000000000000000000000000000000000000000000000000000000);
                self.state.bitboards[BlackPieces as usize] ^= mv.from | mv.to | 0x9000000000000000;
            }
        }
        // Change color
        self.switch_color();

        // Change Castling Rights
        self.update_zobrist_hash_castle_rights();
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
        self.update_zobrist_hash_castle_rights();

        // let eval = self.mg_eval();
        // self.evaluate();
        // if eval != self.mg_eval() {
        //     println!("{} {}", eval, self.mg_eval());
        // }

        self.repetition_tracker.add(self.zobrist_hash());
    }

    pub fn switch_color(&mut self) {
        self.update_zobrist_color();
        self.side_to_move = self.side_to_move.swapped();
        self.update_zobrist_color();
    }
    pub fn switch_color_no_zobrist_change(&mut self) {
        self.side_to_move = self.side_to_move.swapped();
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.side_to_move = new_color;
    }

    // british init???!?!?!!?!
    pub fn init(&mut self) {
        self.gen_zobrist_hash();
        self.evaluate();
        self.repetition_tracker.add(self.zobrist_hash());
    }

    pub fn undo(&mut self) {
        self.repetition_tracker.remove(self.zobrist_hash());
        self.state = self.history.pop().unwrap();
        self.switch_color_no_zobrist_change();
    }

    pub fn reset_hist(&mut self) {
        self.repetition_tracker.reset();
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