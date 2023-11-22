pub use crate::board::*;

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

pub fn chess_to_square(square: String) -> usize {
    if square.chars().collect::<Vec<char>>().len() < 2 {return 65;}

    let x: char  = square.chars().collect::<Vec<char>>()[1];
    let y: usize = chesscol_to_square(square.chars().next().unwrap());

    if x.is_ascii_digit() {
        8*(x.to_digit(10).unwrap() as usize - 1) + y
    } else {
        // Return number >64 if invalid
        65
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
    65
}

// something like ..001000 to 4
pub fn bbsquare_to_chess(square: u64) -> String {
    square_to_chess(square.trailing_zeros() as usize)
}

pub fn square_to_chess(square: usize) -> String {
    let pos_to_char: [&str; 8] = ["h", "g", "f", "e", "d", "c", "b", "a"];

    let x: usize = square % 8;
    let y: usize = (square - x) / 8;

    String::from(pos_to_char[x]) + &format!("{}", y + 1)
}

// my silly move rep into a2a4 or whatever
pub fn move_to_chess(mv: Move) -> String {
    format!("{}{}{}",bbsquare_to_chess(mv.from),bbsquare_to_chess(mv.to),flag_to_piece(mv.flag))
}