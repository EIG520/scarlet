use std::{fs::OpenOptions, io::{BufWriter, Write}, time::Instant};
use tokio::sync::mpsc::Sender;

use bullet::game::formats::viriformat::{self, chess::{board::GameOutcome, chessmove::MoveFlags, piece::PieceType}};
use viriformat::chess::types;
use rand::random_range;
use scarlet::{board::Board, moves::{Flag, Move, MoveList}, uci::{Searcher, StoredOptions, TranspositionTable}};

pub struct DataGenner {
    virigame: viriformat::dataformat::Game,
    viriboard: viriformat::chess::board::Board,
    scarboard: Board,
    sender: Sender<viriformat::dataformat::Game>,
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<viriformat::dataformat::Game>(1000);

    for _ in 0..16 {
        let snew = sender.clone();

        tokio::spawn(async move {
            let mut dg = DataGenner::new(snew);

            for _ in 0..1000 {
                dg.gen_games(1000, 8, 5000).await;
            }
        });
    }

    let mut file = BufWriter::new(OpenOptions::new().create(true).append(true).open("train7.viri").expect("couldn't open file"));

    let mut num_games = 0;
    let mut num_poses = 0;

    loop {
        if let Some(game) = receiver.recv().await {
            let _ = game.serialise_into(&mut file);
            num_games += 1;
            num_poses += game.moves.len() + 1;

            if num_games % 128 == 0 {
                let _ = file.flush();
            }

            if num_games % 1000 == 0 {
                println!("{num_poses} positions ({num_games} games)");
            }
        }
    }
}

impl DataGenner {
    pub async fn gen_games(&mut self, games: usize, randmoves: usize, softnodes: u128) {
        for _ in 0..games {
            self.gen_game(randmoves, softnodes).await;
        }
    }

    pub async fn gen_game(&mut self, randmoves: usize, softnodes: u128) {
        self.init_random(randmoves);

        while self.viriboard.outcome() == GameOutcome::Ongoing {
            self.play_scarlet_move(softnodes);
        }

        self.virigame.set_outcome(self.viriboard.outcome());

        let mut sgame = viriformat::dataformat::Game::new(&self.virigame.initial_position.unpack().0);
        sgame.moves = self.virigame.moves.clone();
        sgame.set_outcome(self.viriboard.outcome());
        let _ = self.sender.send(sgame).await;
    }

    pub fn init_random(&mut self, moves: usize) {
        self.viriboard.set_startpos();
        self.scarboard = Board::new();
        
        for _ in 0..moves {
            if self.viriboard.outcome() != GameOutcome::Ongoing {
                break;
            }

            let mut mvs = MoveList::default();
            self.scarboard.gen_legal_moves(&mut mvs, false);

            let mv = mvs.moves[random_range(0..mvs.pos)];

            // println!("{}", move_to_chess(mv));
    
            self.viriboard.make_move_simple(scarmv_to_virimv(mv));
            self.scarboard.make_move(&mv);

            // println!("{}", self.viriboard.to_string());
            // self.scarboard.output();
        }

        self.virigame = viriformat::dataformat::Game::new(&self.viriboard);
    }

    pub fn play_scarlet_move(&mut self, softnodes: u128) {
        let mut tt = TranspositionTable::new(softnodes.min(1000000) as usize);
        let mut searcher = Searcher::new(&mut self.scarboard, &mut tt, StoredOptions { use_tt: true });
        searcher.set_search_ms(10);

        let timer = Instant::now();
        let mut depth = 0;
        while searcher.nodes() < softnodes {
            depth += 1;

            searcher.search(depth, -30000, 30000, 0, timer);
        }

        let mv = searcher.root_best_move();
        // println!("{}", move_to_chess(mv));

        self.virigame.add_move(scarmv_to_virimv(mv), searcher.white_eval() as i16);
        self.viriboard.make_move_simple(scarmv_to_virimv(mv));
        self.scarboard.make_move(&mv);

        // println!("{}", self.viriboard.to_string());
        // self.scarboard.output();
    }
}

impl DataGenner {
    pub fn new(sender: Sender<viriformat::dataformat::Game>) -> Self {        
        Self {
            virigame: viriformat::dataformat::Game::new(&viriformat::chess::board::Board::default()),
            viriboard: viriformat::chess::board::Board::default(),
            scarboard: Board::new(),
            sender,
        }
    }
}

pub fn scarmv_to_virimv(mv: Move) -> viriformat::chess::chessmove::Move {
    // println!("{}",mv.from.trailing_zeros());
    let fromsq = unsafe { types::Square::default().add_unchecked(mv.from.trailing_zeros() as u8 ^ 7)};
    // println!("{}",mv.to.trailing_zeros());
    let tosq = unsafe { types::Square::default().add_unchecked(mv.to.trailing_zeros() as u8 ^ 7) };

    match mv.flag {
        Flag::NoFlag => { viriformat::chess::chessmove::Move::new(fromsq, tosq) }
        
        Flag::WhiteEnPassant => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, tosq, MoveFlags::EnPassant) }
        Flag::BlackEnPassant => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, tosq, MoveFlags::EnPassant) }
        
        Flag::KnightPromotion => { viriformat::chess::chessmove::Move::new_with_promo(fromsq, tosq, PieceType::Knight) }
        Flag::BishopPromotion => { viriformat::chess::chessmove::Move::new_with_promo(fromsq, tosq, PieceType::Bishop) }
        Flag::RookPromotion => { viriformat::chess::chessmove::Move::new_with_promo(fromsq, tosq, PieceType::Rook) }
        Flag::QueenPromotion => { viriformat::chess::chessmove::Move::new_with_promo(fromsq, tosq, PieceType::Queen) }

        Flag::WhiteKingsideCastle => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, types::Square::H1, MoveFlags::Castle) }
        Flag::BlackKingsideCastle => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, types::Square::H8, MoveFlags::Castle) }
        Flag::WhiteQueensideCastle => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, types::Square::A1, MoveFlags::Castle) }
        Flag::BlackQueensideCastle => { viriformat::chess::chessmove::Move::new_with_flags(fromsq, types::Square::A8, MoveFlags::Castle) }        
    }
}