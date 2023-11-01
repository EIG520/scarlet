use std::collections::HashMap;
use std::str::SplitWhitespace;

pub use crate::board;
pub use crate::moves;
pub use crate::tools;
pub use crate::monte;
pub use crate::search;
pub use crate::test;



use self::board::chess_to_move;

pub fn uci() {
    let mut line ;
    for _ in 0..100000 {
        line = String::new();
        let _b = std::io::stdin().read_line(&mut line).unwrap();

        match line.trim() {
            "isready" => {println!("readyok")},
            "uci" => {println!("uciok")},
            "exit" => {return},
            "quit" => {return},
            _ => {
                run_uci(&line);
            },
        }
    }
}


pub fn run_uci(cmd: &String) {
    let mut flds = cmd.split_whitespace();
    
    match flds.next() {
        Some("ucinewgame") => {board::reset_hist();},
        Some("go") => {handle_go(flds, HashMap::new());},
        Some("position") => {handle_position(flds);},
        Some("d") => {board::print_bb(board::get_bitboard(13) | board::get_bitboard(12));},
        _ => {}
    }
}

// TODO: make this better lol
fn handle_go(mut flds: SplitWhitespace<'_>, known_values: HashMap<&str, u128>) {
    let next = flds.next();

    if next.is_none() {
        handle_best(0);
        return;
    }

    match next.unwrap() {
        "wtime" => {handle_wtime(flds, known_values);},
        "btime" => {handle_btime(flds, known_values);},
        "winc" => {handle_winc(flds, known_values);},
        "binc" => {handle_binc(flds, known_values);},
        "perft" => {handle_perft(flds);},
        _ => {handle_go(flds,known_values);}
    }
}

fn handle_wtime(mut flds: SplitWhitespace<'_>, mut known_values: HashMap<&str, u128>) {
    if board::color() == 0 {
        handle_best(flds.next().unwrap().parse::<u128>().unwrap());
        return;
    }

    known_values.insert("wtime", flds.next().unwrap().parse::<u128>().unwrap());
    handle_go(flds, known_values);
}

fn handle_btime(mut flds: SplitWhitespace<'_>, mut known_values: HashMap<&str, u128>) {
    if board::color() == 1 {
        handle_best(flds.next().unwrap().parse::<u128>().unwrap());
        return;
    }

    known_values.insert("btime", flds.next().unwrap().parse::<u128>().unwrap());
    handle_go(flds, known_values);
}

fn handle_winc(mut flds: SplitWhitespace<'_>, mut known_values: HashMap<&str, u128>) {
    known_values.insert("winc", flds.next().unwrap().parse::<u128>().unwrap());
    handle_go(flds, known_values);
}
fn handle_binc(mut flds: SplitWhitespace<'_>, mut known_values: HashMap<&str, u128>) {
    known_values.insert("binc", flds.next().unwrap().parse::<u128>().unwrap());
    handle_go(flds, known_values);
}

fn handle_perft(mut flds: SplitWhitespace<'_>) {
    let fld = flds.next();

    if fld != None {
        let res = fld.unwrap().parse::<i32>();
        if res.is_ok() {
            println!("Nodes: {}", tools::perft(res.unwrap(), true));
        }
    }
}

fn handle_position(mut flds: SplitWhitespace<'_>) {
    match flds.next().unwrap() {
        "fen" => {handle_fen(flds)},
        "startpos" => {handle_startpos(flds)},
        _ => {}
    }
}

pub fn handle_fen(mut flds: SplitWhitespace<'_>) {
    let mut pos: usize = 0;
    
    // Clear out bitboards
    for i in 0..64 {
        board::del_from_square(i);
    }
    board::reset_hist();

    let first = flds.next().unwrap();

    // Set the pieces
    for c in first.chars() {
        if c.is_digit(10) {
            pos += c.to_digit(10).unwrap() as usize;
        }
        else {
            match c {
                'P' =>  board::set_square(63 - pos, 0),
                'p' =>  board::set_square(63 - pos, 1),
                'N' =>  board::set_square(63 - pos, 2),
                'n' =>  board::set_square(63 - pos, 3),
                'B' =>  board::set_square(63 - pos, 4),
                'b' =>  board::set_square(63 - pos, 5),
                'R' =>  board::set_square(63 - pos, 6),
                'r' =>  board::set_square(63 - pos, 7),
                'Q' =>  board::set_square(63 - pos, 8),
                'q' =>  board::set_square(63 - pos, 9),
                'K' =>  board::set_square(63 - pos, 10),
                'k' =>  board::set_square(63 - pos, 11),
                // We can ignore /s 
                '/' => {pos -= 1}
                // If it's a space, we move on to the next part
                ' ' => {break}
                _ => panic!("Invalid FEN (or I made a bug)")
            }
            pos += 1;
        }
    }

    // Side to move
    if flds.next().unwrap() == "w" {
        board::set_color(0);
    }
    else {
        board::set_color(1);
    }

    // Castling
    let mut crs: u64 = 0;
    for c in flds.next().unwrap().chars() {
        match c {
            'K' => crs += 0b1000,
            'Q' => crs += 0b0100,
            'k' => crs += 0b0010,
            'q' => crs += 0b0001,
            _ => {}
        }
    }
    board::set_bitboard(14, crs);

    // En passant target square
    let ep = board::chess_to_square(flds.next().unwrap().to_string());
    // If chest_to_square returns something > 64, then it is invalid
    // or _ and should mean that the ep bitboard is set to 0
    if ep > 64 {
        board::set_bitboard(16, 0);
    } else {
        board::set_bitboard(16, 1 << ep);
    }

    // Fifty move rule stuff
    flds.next();
    // Move number
    flds.next();

    // do the moves part
    if flds.next().is_some() {
        handle_moves(flds);
    }
}

pub fn handle_startpos(mut flds: SplitWhitespace<'_>) {
    let mut pos: usize = 0;
    
    // Clear out bitboards
    for i in 0..64 {
        board::del_from_square(i);
    }
    board::reset_hist();

    let first = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    // Set the pieces
    for c in first.chars() {
        if c.is_digit(10) {
            pos += c.to_digit(10).unwrap() as usize;
        }
        else {
            match c {
                'P' =>  board::set_square(63 - pos, 0),
                'p' =>  board::set_square(63 - pos, 1),
                'N' =>  board::set_square(63 - pos, 2),
                'n' =>  board::set_square(63 - pos, 3),
                'B' =>  board::set_square(63 - pos, 4),
                'b' =>  board::set_square(63 - pos, 5),
                'R' =>  board::set_square(63 - pos, 6),
                'r' =>  board::set_square(63 - pos, 7),
                'Q' =>  board::set_square(63 - pos, 8),
                'q' =>  board::set_square(63 - pos, 9),
                'K' =>  board::set_square(63 - pos, 10),
                'k' =>  board::set_square(63 - pos, 11),
                // We can ignore /s 
                '/' => {pos -= 1}
                // If it's a space, we move on to the next part
                ' ' => {break}
                _ => panic!("Invalid FEN (or I made a bug)")
            }
            pos += 1;
        }
    }

    // Side to move
    board::set_color(0);

    // Castling
    board::set_bitboard(14, 0b1111);

    // En passant target square
    board::set_bitboard(16, 0);

    // Fifty move rule stuff
    board::set_bitboard(15, 0);

    // Move number

    // do the moves part
    if flds.next().is_some() {
        handle_moves(flds);
    }
}

fn handle_moves(mut flds: SplitWhitespace<'_>) {
    let mut nxt = flds.next();

    while nxt.is_some() {
        let mv = chess_to_move(nxt.unwrap().to_string());
        board::movebb(mv.0, mv.1, mv.2, mv.3);

        nxt = flds.next();
    }

}

fn handle_best(time: u128) {
    println!("bestmove {}", board::move_to_chess(search::bestmove(time / 30)));
}