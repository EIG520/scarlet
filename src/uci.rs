
use std::error::Error;
use std::str::SplitWhitespace;

pub use crate::board::*;
pub use crate::utils::*;
pub use crate::search::*;

use std::fs::File;
use std::io::{self, BufRead};

pub struct UciHandler {
    board: Board,
}

impl Default for UciHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl UciHandler {
    pub fn new() -> Self {
        Self {
            board: Board::new()
        }
    }

    pub fn uci(&mut self) {
        let mut line: String;

        loop {
            line = String::new();
            let _b = std::io::stdin().read_line(&mut line).unwrap();
    
            match self.handle_once(&mut line.split_whitespace()) {
                Ok(_) => {},
                Err(_) => break,
            }
        }
    }

    pub fn handle_once(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()> {
        match command.next() {
            Some("ucinewgame") => Ok(()),
            Some("isready") => {println!("readyok"); Ok(())},
            Some("uci") => {println!("uciok"); Ok(())},

            Some("exit") => Err(()),
            Some("quit") => Err(()),

            Some("go") => self.handle_go(command),
            Some("position") => self.handle_position(command),

            Some("d") => {print_bb(self.board.get_bitboard(PieceType::WhitePieces) | self.board.get_bitboard(PieceType::BlackPieces));println!("{}", self.board.zobrist_hash());println!("{}", self.board.eval());Ok(())}
            _ => Ok(())
        }
    }

    pub fn handle_go(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()> {
        let mut next = command.next();

        while next.is_some() {
            match next {
                Some("perft") => {return self.handle_perft(command)},
                Some("depth") => {return self.handle_depth(command)},
                Some(name) if name == match self.board.color() {Color::White => "wtime", Color::Black => "btime"} => {return self.handle_time(command)}
                _ => {}
            }

            next = command.next();
        }

        Ok(())
    }
    
    pub fn handle_time(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()> {
        match command.next() {
            Some(x) if x.parse::<u128>().is_ok() => {
                let mut searcher: Searcher = Searcher::new(&mut self.board);
                searcher.search_for_ms(x.parse::<u128>().unwrap() / 30);
                Ok(())
            },
            _ => {Err(())}
        }
    }

    pub fn handle_depth(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()> {

        let next = command.next();
        match next {
            Some(a) if a.parse::<i32>().is_ok() => {
                let mut searcher: Searcher = Searcher::new(&mut self.board);
                println!("{}",move_to_chess(searcher.search_to_depth(a.parse::<i32>().unwrap())));
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn handle_perft(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()> {
        let next = command.next();

        match next {
            Some(depth_str) => {
                match depth_str.parse::<u64>() {
                    Ok(_) => {self.board.perft(depth_str.parse::<u64>().unwrap());Ok(())},
                    Err(_) => {Err(())}
                }
            },
            _ => Err(()),
        }
    }

    pub fn handle_position(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()>{
        let next = command.next();

        match next {
            Some("fen") => self.handle_fen(command),
            Some("startpos") => self.handle_startpos(command),
            _ => Err(()),
        }
    }
    pub fn handle_startpos(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()>{
        let mut pos: usize = 0;
    
        // Clear out bitboards
        self.board.clear();
    
        let first = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    
        // Set the pieces
        for c in first.chars() {
            if c.is_ascii_digit() {
                pos += c.to_digit(10).unwrap() as usize;
            }
            else {
                match c {
                    'P' =>  self.board.add_to_square(63 - pos, PieceType::WhitePawn),
                    'p' =>  self.board.add_to_square(63 - pos, PieceType::BlackPawn),
                    'N' =>  self.board.add_to_square(63 - pos, PieceType::WhiteKnight),
                    'n' =>  self.board.add_to_square(63 - pos, PieceType::BlackKnight),
                    'B' =>  self.board.add_to_square(63 - pos, PieceType::WhiteBishop),
                    'b' =>  self.board.add_to_square(63 - pos, PieceType::BlackBishop),
                    'R' =>  self.board.add_to_square(63 - pos, PieceType::WhiteRook),
                    'r' =>  self.board.add_to_square(63 - pos, PieceType::BlackRook),
                    'Q' =>  self.board.add_to_square(63 - pos, PieceType::WhiteQueen),
                    'q' =>  self.board.add_to_square(63 - pos, PieceType::BlackQueen),
                    'K' =>  self.board.add_to_square(63 - pos, PieceType::WhiteKing),
                    'k' =>  self.board.add_to_square(63 - pos, PieceType::BlackKing),
                    // We can ignore /s 
                    '/' => {pos -= 1}
                    // If it's a space, we move on to the next part
                    ' ' => {break}
                    _ => {return Err(())}
                }
                pos += 1;
            }
        }
    
        // Side to move
        self.board.set_color(Color::White);
    
        // Castling
        let mut crs: u64 = 0;
        for c in "KQkq".chars() {
            match c {
                'K' => crs += 0b1000,
                'Q' => crs += 0b0100,
                'k' => crs += 0b0010,
                'q' => crs += 0b0001,
                '-' => {},
                _ => {return Err(());}
            }
        }
        
        self.board.set_bitboard(PieceType::CastleRights, crs);
    
        // En passant target square
        let ep = chess_to_square("-".to_string());
        // If chest_to_square returns something > 64, then it is invalid
        // or _ and should mean that the ep bitboard is set to 0
        if ep > 64 {
            self.board.set_bitboard(PieceType::EnPassant, 0);
        } else {
            self.board.set_bitboard(PieceType::EnPassant, 1 << ep);
        }

        self.board.set_move_count(0);

        self.board.init();

        match command.next() {
            Some("moves") => {
                let mv = command.next();
                self.handle_moves(command, mv)
            },
            _ => {
                Ok(())
            }
        }

    }

    pub fn handle_fen(&mut self, command: &mut SplitWhitespace<'_>) -> Result<(), ()>{
        let mut pos: usize = 0;
    
        // Clear out bitboards
        self.board.clear();
    
        let first = command.next().unwrap();
    
        // Set the pieces
        for c in first.chars() {
            if c.is_ascii_digit() {
                pos += c.to_digit(10).unwrap() as usize;
            }
            else {
                match c {
                    'P' =>  self.board.add_to_square(63 - pos, PieceType::WhitePawn),
                    'p' =>  self.board.add_to_square(63 - pos, PieceType::BlackPawn),
                    'N' =>  self.board.add_to_square(63 - pos, PieceType::WhiteKnight),
                    'n' =>  self.board.add_to_square(63 - pos, PieceType::BlackKnight),
                    'B' =>  self.board.add_to_square(63 - pos, PieceType::WhiteBishop),
                    'b' =>  self.board.add_to_square(63 - pos, PieceType::BlackBishop),
                    'R' =>  self.board.add_to_square(63 - pos, PieceType::WhiteRook),
                    'r' =>  self.board.add_to_square(63 - pos, PieceType::BlackRook),
                    'Q' =>  self.board.add_to_square(63 - pos, PieceType::WhiteQueen),
                    'q' =>  self.board.add_to_square(63 - pos, PieceType::BlackQueen),
                    'K' =>  self.board.add_to_square(63 - pos, PieceType::WhiteKing),
                    'k' =>  self.board.add_to_square(63 - pos, PieceType::BlackKing),
                    // We can ignore /s 
                    '/' => {pos -= 1}
                    // If it's a space, we move on to the next part
                    ' ' => {break}
                    _ => {return Err(())}
                }
                pos += 1;
            }
        }
    
        // Side to move
        match command.next() {
            Some("w") => {self.board.set_color(Color::White)},
            Some("b") => {self.board.set_color(Color::Black)},
            _ => {return Err(());},
        }
    
        // Castling
        let mut crs: u64 = 0;
        for c in command.next().unwrap().chars() {
            match c {
                'K' => crs += 0b1000,
                'Q' => crs += 0b0100,
                'k' => crs += 0b0010,
                'q' => crs += 0b0001,
                '-' => {},
                _ => {return Err(());}
            }
        }
        
        self.board.set_bitboard(PieceType::CastleRights, crs);
    
        // En passant target square
        let ep = chess_to_square(command.next().unwrap().to_string());
        // If chest_to_square returns something > 64, then it is invalid
        // or _ and should mean that the ep bitboard is set to 0
        if ep > 64 {
            self.board.set_bitboard(PieceType::EnPassant, 0);
        } else {
            self.board.set_bitboard(PieceType::EnPassant, 1 << ep);
        }

        match (command.next(), command.next()) {
            (Some("moves"), m) => {self.board.init();self.handle_moves(command, m)},
            (Some(a), Some(b)) if a.parse::<u64>().is_ok() && b.parse::<u64>().is_ok() => {
                self.board.set_move_count(b.parse::<u64>().unwrap());
                command.next();
                let f = command.next();
                self.board.init();
                self.handle_moves(command, f)
            },
            (None, None) => {self.board.init();Ok(())},
            _ => {self.board.init();Err(())}
        }
    }

    pub fn handle_moves(&mut self, command: &mut SplitWhitespace<'_>, first_move: Option<&str>) -> Result<(), ()>{
        let mut next: Option<&str> = first_move;

        while next.is_some() {
            self.board.make_move(&self.board.chess_to_move(String::from(next.unwrap())));
            next = command.next();
        }

        Ok(())
    }


}



// Return -1 if test passed, otherwise return depth it failed at
pub fn test_movegen(fen: String, node_counts: Vec<i64>) -> Result<i32, ()>{
    let mut uci: UciHandler = UciHandler::new();

    if let Err(()) = uci.handle_once(&mut format!("position fen {}", fen).split_whitespace()) {return Err(())}

    if let Err(()) = uci.handle_once(&mut format!("position fen {}", fen).split_whitespace()) {return Err(())}

    for i in 0..node_counts.len() as u64 {
        if node_counts[i as usize] == -1 {continue;}

        let ncs = uci.board.sub_perft(i + 1);
        if ncs != node_counts[i as usize] as u64{
            println!("Perft {} failed ({})", i+1, fen);
            return Ok(i as i32 + 1);
        }
        println!("Perft {} passed ({})", i+1, fen);
    }
    Ok(-1)
}

pub fn test_movegen_on_suite(suite_filename: &str) -> Result<(), Box::<dyn Error>> {
    let file = File::open(suite_filename)?;

    for line in io::BufReader::new(file).lines() {
        let ln = line?;
        let mut info = ln
            .split(';')
            .map(|i| i.to_string());

        let fen = info.next().unwrap();
        let mut node_counts: Vec<i64> = vec![];

        for token in info {
            let mut option = token.split_whitespace();

            let index = option.next().unwrap()[1..].parse::<u64>().unwrap();

            while index != node_counts.len() as u64 + 1{
                node_counts.push(-1);
            }

            match option.next() {
                Some(cnt) if cnt.parse::<i64>().is_ok() => {
                    node_counts.push(cnt.parse::<i64>().unwrap());
                }
                _ => {}
            }
        }
        println!("{}", fen);
        println!("{:?}", node_counts);
        match test_movegen(fen, node_counts) {
            Err(()) => {return Err(Box::<dyn Error>::from("something bad happened"))},
            Ok(a) if a != -1 => {return Ok(())},
            _ => {},
        }
    }
    Ok(())
}