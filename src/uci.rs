use std::str::SplitWhitespace;

pub use crate::board;
pub use crate::moves;
pub use crate::tools;

pub fn uci() {
    let mut line ;
    loop {
        line = String::new();
        let _b = std::io::stdin().read_line(&mut line).unwrap();
        run_uci(&line);
    }
}

pub fn run_uci(cmd: &String) {
    let mut flds = cmd.split_whitespace().into_iter();
    
    match flds.next().unwrap() {
        "uci" => {println!("uciok")},
        "go" => {handle_go(flds);},
        "position" => {handle_position(flds);},
        _ => {}
    }
}

fn handle_go(mut flds: SplitWhitespace<'_>) {
    match flds.next().unwrap() {
        "perft" => {handle_perft(flds);},
        _ => {}
    }
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

    // Set the pieces
    for c in flds.next().unwrap().chars() {
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

    // TODO: This (stinky)


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
}