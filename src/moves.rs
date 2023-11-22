pub use crate::board::*;

#[macro_export]
macro_rules! bitloop {
    ($x:tt$y:block) => {
        let mut $x = $x;
        while $x > 0 {
            $y
            $x = $x & ($x - 1);
        }
    }
}
pub(crate) use bitloop;

impl Board {
    pub fn gen_legal_moves(&mut self, moves: &mut MoveList){
        // Make some shared data
        self.gen_checkmask();
        self.gen_hit_squares();
        moves.clear();

        // Just call these three and bam
        self.unpinned_legal_moves(moves);
        self.rook_pinned_legal_moves(moves);
        self.bishop_pinned_legal_moves(moves);
    }

    pub fn unpinned_legal_moves(&mut self, moves: &mut MoveList){
        // Checkmask
        let attacked_squares = self.attacked();
        let checkmask = self.checkmask();

        // Pinmasks
        let pinmask = self.rook_pinmask() | self.bishop_pinmask();


        let pawns = self.get_bitboard(PieceType::WhitePawn.shiftedby(self.color())) & !pinmask;
        match self.color() {
            Color::White => {bitloop!(pawns{
                let bbmoves = self.wpawn_bbmoves(pawns.blsi().trailing_zeros() as usize) & checkmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.wpawn_epmoves(pawns.blsi().trailing_zeros() as usize) & (checkmask << 8);

                    if epmove > 0 {
                        if self.verify_ep(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::WhitePawn, flag : Flag::WhiteEnPassant}) {
                            moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::WhitePawn, flag : Flag::WhiteEnPassant});
                        }
                    }
                }
            });},
            Color::Black => {bitloop!(pawns{
                let bbmoves = self.bpawn_bbmoves(pawns.blsi().trailing_zeros() as usize)& checkmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.bpawn_epmoves(pawns.blsi().trailing_zeros() as usize) & (checkmask >> 8);

                    if epmove > 0 {
                        if self.verify_ep(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::BlackPawn, flag : Flag::BlackEnPassant}) {
                            moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::BlackPawn, flag : Flag::BlackEnPassant});
                        }
                    }
                }
            });},
        }
        
        let knights = self.get_bitboard(PieceType::WhiteKnight.shiftedby(self.color())) & !pinmask;
        bitloop!(knights{
            let bbmoves = self.knight_bbmoves(knights.blsi().trailing_zeros() as usize)& checkmask;
            bitloop!(bbmoves{
                moves.push(Move { from: knights.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteKnight.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
        let bishops = self.get_bitboard(PieceType::WhiteBishop.shiftedby(self.color())) & !pinmask;
        bitloop!(bishops{
            let bbmoves = self.bishop_bbmoves(bishops.blsi().trailing_zeros() as usize)& checkmask;
            bitloop!(bbmoves{
                moves.push(Move { from: bishops.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteBishop.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
        let rooks = self.get_bitboard(PieceType::WhiteRook.shiftedby(self.color())) & !pinmask;
        bitloop!(rooks{
            let bbmoves = self.rook_bbmoves(rooks.blsi().trailing_zeros() as usize)& checkmask;
            bitloop!(bbmoves{
                moves.push(Move { from: rooks.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteRook.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
        let queens = self.get_bitboard(PieceType::WhiteQueen.shiftedby(self.color())) & !pinmask;
        bitloop!(queens{
            let bbmoves = self.queen_bbmoves(queens.blsi().trailing_zeros() as usize)& checkmask;
            bitloop!(bbmoves{
                moves.push(Move { from: queens.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteQueen.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });        
        let kings = self.get_bitboard(PieceType::WhiteKing.shiftedby(self.color()));
        bitloop!(kings{
            let bbmoves = self.king_bbmoves(kings.blsi().trailing_zeros() as usize) & !attacked_squares;
            bitloop!(bbmoves{
                moves.push(Move { from: kings.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteKing.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
        // Castling
        if checkmask == u64::MAX {
            let open = !(self.get_bitboard(PieceType::WhitePieces) | self.get_bitboard(PieceType::BlackPieces));


            if (self.get_bitboard(PieceType::CastleRights) & 0b1000 > 0) && (attacked_squares & 0b0110 == 0) && (open & 0b0110 == 0b0110) && (self.color() == Color::White){
                moves.push(Move { from: 0b1000, to: 0b0010, piece_type: PieceType::WhiteKing, flag: Flag::WhiteKingsideCastle});
            }
            if (self.get_bitboard(PieceType::CastleRights) & 0b0100 > 0) && (attacked_squares & 0b00110000 == 0) && (open & 0b01110000 == 0b01110000) && (self.color() == Color::White){
                moves.push(Move { from: 0b1000, to: 0b00100000, piece_type: PieceType::WhiteKing, flag: Flag::WhiteQueensideCastle});
            }

            if (self.get_bitboard(PieceType::CastleRights) & 0b0010 > 0) && (attacked_squares & 0x600000000000000 == 0) && (open & 0x600000000000000 == 0x600000000000000) &&  (self.color() == Color::Black){
                moves.push(Move { from: 0x800000000000000, to: 0x200000000000000, piece_type: PieceType::BlackKing, flag: Flag::BlackKingsideCastle});
            }
            if (self.get_bitboard(PieceType::CastleRights) & 0b0001 > 0) && (attacked_squares & 0x3000000000000000 == 0) && (open & 0x7000000000000000 == 0x7000000000000000) && (self.color() == Color::Black){
                moves.push(Move { from: 0x800000000000000, to: 0x2000000000000000, piece_type: PieceType::BlackKing, flag: Flag::BlackQueensideCastle});
            }
        }
    }

    pub fn rook_pinned_legal_moves(&mut self, moves: &mut MoveList){
        let checkmask = self.checkmask();

        let pinmask = self.rook_pinmask();

        let pawns = self.get_bitboard(PieceType::WhitePawn.shiftedby(self.color())) & pinmask;
        match self.color() {
            Color::White => {bitloop!(pawns{
                let bbmoves = self.wpawn_bbmoves(pawns.blsi().trailing_zeros() as usize) & checkmask & pinmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.wpawn_epmoves(pawns.blsi().trailing_zeros() as usize) & (checkmask >> 8) & pinmask;
                    if epmove > 0 {
                        moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::WhitePawn, flag : Flag::WhiteEnPassant});
                    }
                }
            });},
            Color::Black => {bitloop!(pawns{
                let bbmoves = self.bpawn_bbmoves(pawns.blsi().trailing_zeros() as usize)& checkmask & pinmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.bpawn_epmoves(pawns.blsi().trailing_zeros() as usize) &  (checkmask << 8) & pinmask;
                    if epmove > 0 {
                        moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::BlackPawn, flag : Flag::BlackEnPassant});
                    }
                }
            });},
        }
        
        let rooks = self.get_bitboard(PieceType::WhiteRook.shiftedby(self.color())) & pinmask;
        bitloop!(rooks{
            let bbmoves = self.rook_bbmoves(rooks.blsi().trailing_zeros() as usize) & checkmask & pinmask;
            bitloop!(bbmoves{
                moves.push(Move { from: rooks.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteRook.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });

        let queens = self.get_bitboard(PieceType::WhiteQueen.shiftedby(self.color())) & pinmask;
        bitloop!(queens{
            // rook-pinned queens can only move like rooks
            let bbmoves = self.rook_bbmoves(queens.blsi().trailing_zeros() as usize)& checkmask & pinmask;
            bitloop!(bbmoves{
                moves.push(Move { from: queens.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteQueen.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
    }

    pub fn bishop_pinned_legal_moves(&mut self, moves: &mut MoveList) {
        let checkmask = self.checkmask();

        let pinmask = self.bishop_pinmask();

        let pawns = self.get_bitboard(PieceType::WhitePawn.shiftedby(self.color())) & pinmask;
        match self.color() {
            Color::White => {bitloop!(pawns{
                let bbmoves = self.wpawn_bbmoves(pawns.blsi().trailing_zeros() as usize) & checkmask & pinmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhitePawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.wpawn_epmoves(pawns.blsi().trailing_zeros() as usize) & (checkmask >> 8) & pinmask;
                    if epmove > 0 {
                        if self.verify_ep(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::WhitePawn, flag : Flag::WhiteEnPassant}) {
                            moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::WhitePawn, flag : Flag::WhiteEnPassant});
                        }
                    }
                }
            });},
            Color::Black => {bitloop!(pawns{
                let bbmoves = self.bpawn_bbmoves(pawns.blsi().trailing_zeros() as usize)& checkmask & pinmask;
                if bbmoves & 0xFF000000000000FF > 0 {
                    bitloop!(bbmoves{
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::KnightPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::BishopPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::RookPromotion});
                        moves.push(Move {from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag: Flag::QueenPromotion});
                    });
                } else {
                    bitloop!(bbmoves{
                        moves.push(Move { from: pawns.blsi(), to: bbmoves.blsi(), piece_type: PieceType::BlackPawn, flag : Flag::NoFlag});
                    });
                    let epmove = self.bpawn_epmoves(pawns.blsi().trailing_zeros() as usize) & (checkmask << 8) & pinmask;
                    if epmove > 0 {
                        if self.verify_ep(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::BlackPawn, flag : Flag::BlackEnPassant}) {
                            moves.push(Move { from: pawns.blsi(), to: epmove, piece_type: PieceType::BlackPawn, flag : Flag::BlackEnPassant});
                        }
                    }
                }
            });},
        }
        let bishops = self.get_bitboard(PieceType::WhiteBishop.shiftedby(self.color())) & pinmask;
        bitloop!(bishops{
            let bbmoves = self.bishop_bbmoves(bishops.blsi().trailing_zeros() as usize)& checkmask & pinmask;
            bitloop!(bbmoves{
                moves.push(Move { from: bishops.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteBishop.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });
        let queens = self.get_bitboard(PieceType::WhiteQueen.shiftedby(self.color())) & pinmask;
        bitloop!(queens{
            // bishop-pinned queens can only move like bishops
            let bbmoves = self.bishop_bbmoves(queens.blsi().trailing_zeros() as usize)& checkmask & pinmask;
            bitloop!(bbmoves{
                moves.push(Move { from: queens.blsi(), to: bbmoves.blsi(), piece_type: PieceType::WhiteQueen.shiftedby(self.color()), flag: Flag::NoFlag});
            });
        });        
    }


}

