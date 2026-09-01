pub use crate::uci::*;
use std::time::Instant;

const MAX_PLY: usize = 128;

impl Board {
    pub fn perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::default();
        self.gen_legal_moves(&mut moves, false);

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i].0;

            self.make_move(&mv);

            let subcount = self.sub_perft(depth - 1);

            self.undo();

            count += subcount;

            println!("{}: {}", move_to_chess(mv), subcount);
        }
        println!("nodes: {}", count);
        count
    }
    pub fn sub_perft(&mut self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::default();
        self.gen_legal_moves(&mut moves, false);

        if depth == 1 {
            return moves.pos as u64;
        }

        let mut count: u64 = 0;

        for i in 0..moves.pos {
            let mv = moves.moves[i].0;

            self.make_move(&mv);

            count += self.sub_perft(depth - 1);

            self.undo();
        }
        count
    }
}

#[derive(Clone, Copy)]
pub struct SearchStackEntry {
    pub mv: Move,
    pub capt: bool,
    pub excluded: Move
}

pub struct Searcher<'a> {
    board: &'a mut Board,
    root_best: Move,
    search_best: Move,
    root_best_eval: i32,
    search_best_eval: i32,
    search_ms: u128,
    nodes: u128,
    transposition_table: &'a mut TranspositionTable,
    history_table: HistoryTable,
    options: StoredOptions,

    search_stack: [SearchStackEntry; MAX_PLY],
}

impl<'a> Searcher<'a> {
    pub fn new(
        board: &'a mut Board,
        transposition_table: &'a mut TranspositionTable,
        options: StoredOptions,
    ) -> Self {
        Self {
            board,
            root_best: Move::null(),
            search_best: Move::null(),
            root_best_eval: -30000,
            search_best_eval: -30000,
            search_ms: 0,
            nodes: 0,
            transposition_table,
            history_table: HistoryTable::default(),
            options,

            search_stack: [SearchStackEntry {
                mv: Move::null(),
                capt: false,
                excluded: Move::null()
            }; 128],
        }
    }

    pub fn search(
        &mut self,
        mut depth: i32,
        mut alpha: i32,
        beta: i32,
        ply: usize,
        timer: Instant,
    ) -> i32 {
        if depth <= 0 {
            return self.qsearch(alpha, beta, ply);
        }

        self.nodes += 1;

        let root: bool = ply == 0;
        let incheck = self.board.in_check();
        let pv = alpha != beta - 1;
        let reduce = !pv && !incheck;
        let excluded = self.search_stack[ply].excluded != Move::null();

        if self.board.upcoming_draw() && !root {
            return 0;
        }

        let mut tt_entry: Option<TranspositionInfo> = None;

        if self.options.use_tt {
            tt_entry = self.transposition_table.probe(self.board);
        }

        let mut stat = self.board.gen_eval();
        let mut maybe_singular = false;

        if !excluded && let Some(entry) = tt_entry {
            let score = entry.score;

            if !pv
                && entry.depth as i32 >= depth
                && match entry.fail {
                    Fail::NoFail => true,
                    Fail::FailHigh => score >= beta,
                    Fail::FailLow => score <= alpha,
                    Fail::None => false, // not possible
                }
            {
                return score;
            }

            // Use TT eval instead of static evaluation
            stat = score;

            // Detect possible singularity
            maybe_singular = depth > 7
                && entry.depth as i32 >= depth - 3
                && entry.fail != Fail::FailLow
                && score.abs() < 20000
        } else if depth > 4 {
            depth -= 1;
        }

        if ply >= MAX_PLY {
            println!("too much singuiling");
            return stat;
        }

        let improving = !incheck
            && self.board.hist_len() >= 2
            && self.board.state().eval() > self.board.get_nth_prev_boardstate(2).eval();

        // Pruning
        if !root && reduce && !excluded {
            // rfp
            if stat - 85 * depth + if improving { 85 } else { 0 } >= beta {
                return stat;
            }

            // null move pruning
            if depth > 2 && stat >= beta {
                self.board.make_null_move();
                self.search_stack[ply as usize] = SearchStackEntry {
                    mv: Move::null(),
                    capt: false,
                    excluded: Move::null()
                };

                let eval = -self.search(
                    (depth * 100 + beta - stat) / 200 - 1,
                    -beta,
                    1 - beta,
                    ply + 1,
                    timer,
                );

                self.board.unmake_null_move();

                if eval >= beta {
                    return eval;
                }
            }
        }

        let mut mvs = MoveList::default();
        self.board.gen_legal_moves(&mut mvs, false);

        if let Some(entry) = tt_entry {
            self.board.score_moves_see(
                &mut mvs,
                entry.best_move,
                &self.search_stack,
                &self.history_table,
                ply,
            );
        } else {
            self.board.score_moves_see(
                &mut mvs,
                Move::null(),
                &self.search_stack,
                &self.history_table,
                ply,
            );
        }

        // Main Search
        let mut best = -30000;
        let mut best_move = Move::null();
        let mut mvtype = Fail::FailLow;

        for i in 0..mvs.pos {
            if self.nodes % 2048 == 0
                && self.search_ms != 0
                && timer.elapsed().as_millis() >= self.search_ms
            {
                return 30000;
            }

            self.board.find_best_nth(&mut mvs, i);

            let (mv, _score) = mvs.moves[i];

            if mv == self.search_stack[ply].excluded {
                continue;
            }

            let is_capture = mv.to
                & (self.board.get_bitboard(PieceType::WhitePieces)
                    | self.board.get_bitboard(PieceType::BlackPieces))
                != 0;
            let is_qpromo = mv.flag == Flag::QueenPromotion;

            // moveloop pruning (thank you amber21!!!)
            if !root && !incheck && best >= -20000 {
                // late move pruning
                if !pv
                    && !is_capture
                    && !is_qpromo
                    && i as i32 > 5 + depth * depth
                    && alpha.abs() < 2000
                    && beta < 20000
                {
                    break;
                }

                // futility pruning
                if !is_capture
                    && !is_qpromo
                    && stat + 256 + 128 * depth < alpha
                    && alpha.abs() < 2000
                    && depth <= 6
                {
                    break;
                }
            }
            
            let mut ext = 0;

            if maybe_singular && i == 0 && !root && !excluded {
                let margin = depth * 2;
                let sdepth = (depth - 1) / 2;

                self.search_stack[ply].excluded = mv;
                let sing_score = self.search(sdepth, stat - margin - 1, stat - margin, ply, timer);
                self.search_stack[ply].excluded = Move::null();

                if sing_score < stat - margin {
                    // if ext == 0 {
                    //     println!("sunguilu extEE!!! {} < {} {}", sing_score, stat - margin, stat);
                    // }
                    ext = 1;
                }
            }


            self.board.make_move(&mv);
            self.search_stack[ply] = SearchStackEntry {
                mv,
                capt: is_capture,
                excluded: self.search_stack[ply].excluded
            };

            if self.board.in_check() {
                ext = 1;
            }

            let newdepth = depth - 1 + ext;

            let mut eval = 67;

            let ireq = if pv { 6 } else { 2 };

            if i > ireq && depth >= 2 && (!is_capture && !is_qpromo) {
                let reduction = (1.0 + (depth as f32).ln() * (i as f32).ln() / 2.0
                    - self.history_table.probe(&self.search_stack, mv, ply) as f32 / 200.0)
                    .floor() as i32;

                let reduced = (newdepth - reduction).clamp(0, depth - 1);

                eval = -self.search(reduced, -alpha - 1, -alpha, ply + 1, timer);

                if eval > alpha && reduced < newdepth {
                    eval = -self.search(newdepth, -alpha - 1, -alpha, ply + 1, timer);
                }
            } else if !pv || i > 0 {
                eval = -self.search(newdepth, -alpha - 1, -alpha, ply + 1, timer);
            }

            if pv && (i == 0 || eval > alpha) {
                eval = -self.search(newdepth, -beta, -alpha, ply + 1, timer);
            }

            self.board.undo();

            if eval > best {
                best = eval;
                best_move = mv;

                if root {
                    self.search_best = mv;
                    self.search_best_eval = eval;
                }

                // Alpha beta pruning
                if eval > alpha {
                    alpha = eval;
                    mvtype = Fail::NoFail;
                }
                if alpha >= beta {
                    mvtype = Fail::FailHigh;

                    if !is_capture {
                        self.history_table.add_killer(mv, ply as i32);
                        self.history_table
                            .apply_delta(&self.search_stack, mv, depth * depth, ply);

                        for j in 0..i {
                            let mv2 = mvs.moves[j].0;
                            let is_capture_2 = mv2.to
                                & (self.board.get_bitboard(PieceType::WhitePieces)
                                    | self.board.get_bitboard(PieceType::BlackPieces))
                                != 0;

                            if !is_capture_2 {
                                self.history_table.apply_delta(
                                    &self.search_stack,
                                    mv2,
                                    -depth * depth,
                                    ply,
                                );
                            } else {
                                self.history_table.apply_delta_tactical(
                                    mv2,
                                    self.board.piece_on_sq(mv2.to.trailing_zeros() as usize),
                                    -depth * depth,
                                );
                            }
                        }
                    } else {
                        self.history_table.apply_delta_tactical(
                            mv,
                            self.board.piece_on_sq(mv.to.trailing_zeros() as usize),
                            depth * depth,
                        );

                        for j in 0..i {
                            let mv2 = mvs.moves[j].0;
                            let is_capture_2 = mv2.to
                                & (self.board.get_bitboard(PieceType::WhitePieces)
                                    | self.board.get_bitboard(PieceType::BlackPieces))
                                != 0;

                            if is_capture_2 {
                                self.history_table.apply_delta_tactical(
                                    mv2,
                                    self.board.piece_on_sq(mv2.to.trailing_zeros() as usize),
                                    -depth * depth,
                                );
                            }
                        }
                    }

                    break;
                }
            }
        }

        if excluded && mvs.pos < 2 {
            return - 30000;
        }

        if mvs.pos == 0 {
            // in check & no moves = mate
            if incheck {
                return ply as i32 - 30000;
            }
            return 0;
        }

        if self.options.use_tt && !excluded {
            self.transposition_table.add(
                self.board,
                depth as i8,
                best,
                if mvtype != Fail::FailLow || tt_entry.is_none() {
                    best_move
                } else {
                    tt_entry.unwrap().best_move
                },
                mvtype,
            );
        }

        if root
            && (timer.elapsed().as_millis() < self.search_ms
                || self.search_best_eval > self.root_best_eval)
        {
            self.root_best = self.search_best;
            self.root_best_eval = self.search_best_eval;
        }

        best
    }

    fn qsearch(&mut self, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        self.nodes += 1;

        // let incheck = self.board.in_check();
        // let pv = alpha != beta - 1;

        if self.board.upcoming_draw() {
            return 0;
        }

        let mut tt_entry: Option<TranspositionInfo> = None;

        if self.options.use_tt {
            tt_entry = self.transposition_table.probe(self.board);
        }

        if let Some(entry) = tt_entry {
            let score = entry.score;

            if match entry.fail {
                Fail::NoFail => true,
                Fail::FailHigh => score >= beta,
                Fail::FailLow => score <= alpha,
                Fail::None => false, // not possible
            } {
                return score;
            }
        }

        // Stand pat check
        let stat = self.board.gen_eval();
        if stat >= beta || ply >= MAX_PLY {
            return (stat + beta) / 2;
        }
        if alpha < stat {
            alpha = stat;
        }

        // Move generation
        let mut mvs = MoveList::default();
        self.board.gen_legal_moves(&mut mvs, true);
        if let Some(entry) = tt_entry {
            self.board.score_moves(
                &mut mvs,
                entry.best_move,
                &self.search_stack,
                &self.history_table,
                ply,
            );
        } else {
            self.board.score_moves(
                &mut mvs,
                Move::null(),
                &self.search_stack,
                &self.history_table,
                ply,
            );
        }

        // Main search
        let mut best = stat;

        for i in 0..mvs.pos {
            self.board.find_best_nth(&mut mvs, i);

            let (mv, _score) = mvs.moves[i];

            if i > 2 {
                break;
            }

            if !self.board.see_threshold(mv, 0) {
                continue;
            }

            let is_capture = mv.to
                & (self.board.get_bitboard(PieceType::WhitePieces)
                    | self.board.get_bitboard(PieceType::BlackPieces))
                != 0;

            self.board.make_move(&mv);
            self.search_stack[ply] = SearchStackEntry {
                mv,
                capt: is_capture,
                excluded: self.search_stack[ply].excluded
            };

            let eval = -self.qsearch(-beta, -alpha, ply + 1);

            self.board.undo();

            if eval > best {
                best = eval;

                if eval > alpha {
                    alpha = eval;
                }

                if alpha >= beta {
                    break;
                }
            }
        }

        if mvs.pos == 0 {
            return stat;
        }

        best
    }

    pub fn search_to_depth(&mut self, depth: i32) -> Move {
        self.board.populate_accumulators();
        self.history_table = HistoryTable::default();
        self.search_ms = 0;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;

        let timer = Instant::now();

        self.search(depth as i32, -30000, 30000, 0, timer);

        if timer.elapsed().as_millis() > 0 {
            print!(
                "info depth {} nodes {} nps {} score cp {} time {}",
                depth,
                self.nodes,
                1000 * self.nodes / timer.elapsed().as_millis(),
                self.root_best_eval,
                timer.elapsed().as_millis()
            );
        } else {
            print!(
                "info depth {} nodes {} score cp {} time {}",
                depth,
                self.nodes,
                self.root_best_eval,
                timer.elapsed().as_millis()
            );
        }

        print!(" pv");

        let mut tm = self.top_move();

        let mut mvs = 0;
        while tm.is_some() {
            print!(" {}", move_to_chess(tm.unwrap()));

            self.board.make_move(&tm.unwrap());

            tm = self.top_move();

            mvs += 1;
        }

        while mvs > 0 {
            self.board.undo();
            mvs -= 1;
        }

        println!();
        println!("bestmove {}", move_to_chess(self.root_best));

        self.root_best
    }

    pub fn top_move(&self) -> Option<Move> {
        let entry = self.transposition_table.probe(self.board);

        if let Some(info) = entry {
            if info.fail == Fail::NoFail {
                return Some(info.best_move);
            }
        }
        None
    }

    pub fn search_for_ms(&mut self, ms: u128) -> Move {
        self.history_table = HistoryTable::default();
        self.search_ms = ms;
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;
        self.board.populate_accumulators();
        let timer = Instant::now();

        let mut depth = 0;

        // Go deeper and deeper until either mate is found or time is up
        while timer.elapsed().as_millis() < self.search_ms / 13 && self.search_best_eval < 20000 {
            if depth == 100 {
                break;
            }

            depth += 1;
            let peval = self.root_best_eval;
            let mut win_left = 10;
            let mut win_right = 10;
            let mut in_bounds = false;

            for _ in 0..3 {
                self.search(depth, peval - win_left, peval + win_right, 0, timer);

                if self.root_best_eval <= peval - win_left {
                    win_left *= 3;
                } else if self.root_best_eval >= peval + win_right {
                    win_right *= 3;
                } else {
                    in_bounds = true;
                    break;
                }
            }

            if !in_bounds {
                self.search(depth, -30000, 30000, 0, timer);
            }

            if timer.elapsed().as_millis() > 0 {
                print!(
                    "info depth {} nodes {} nps {} score cp {} time {}",
                    depth,
                    self.nodes,
                    1000 * self.nodes / timer.elapsed().as_millis(),
                    self.root_best_eval,
                    timer.elapsed().as_millis()
                );
            } else {
                print!(
                    "info depth {} nodes {} score cp {} time {}",
                    depth,
                    self.nodes,
                    self.root_best_eval,
                    timer.elapsed().as_millis()
                );
            }

            let mut tm = self.top_move();

            let mut mvs = 0;
            while tm.is_some() {
                if self.board.upcoming_draw() {
                    break;
                }

                if mvs == 0 {
                    print!(" pv");
                }

                print!(" {}", move_to_chess(tm.unwrap()));
                self.board.make_move(&tm.unwrap());
                tm = self.top_move();
                mvs += 1;
            }

            while mvs > 0 {
                self.board.undo();
                mvs -= 1;
            }

            println!();
        }
        println!("bestmove {}", move_to_chess(self.root_best));
        self.root_best
    }

    pub fn reset_info(&mut self) {
        self.history_table = HistoryTable::default();
        self.root_best_eval = -30000;
        self.search_best_eval = -30000;
        self.nodes = 0;
    }

    pub fn nodes(&mut self) -> u128 {
        self.nodes
    }

    pub fn root_best_move(&mut self) -> Move {
        self.root_best
    }

    pub fn root_eval(&mut self) -> i32 {
        self.root_best_eval
    }

    pub fn white_eval(&mut self) -> i32 {
        self.root_eval()
            * if self.board.color() == Color::White {
                1
            } else {
                -1
            }
    }

    pub fn set_search_ms(&mut self, ms: u128) {
        self.search_ms = ms;
    }
}
