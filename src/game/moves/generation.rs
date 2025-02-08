use crate::game::structs::bitboard::Bitboard;
use crate::game::structs::{board::Board, piece::Piece, color::{Color, Castling}};
use crate::shared::errors::ChessError;
use crate::shared::statics::masks::KING_MASKS;

use super::move_struct::Flag;
use super::{individual::*, move_struct::Move};

use Piece::*;
use Color::*;
use Castling::*;

impl Board{
    /// Generates a list of all pseudolegal moves in a current position
    fn gen_moves(&self) -> Vec<Move> {
        let mut res = vec![];
        // TODO: there should be only 1 king, so iterating over the bitboard shouldnt be necessary
        // but right now position isnt validated, so more than 1 king is possible
        // needs to be implemented later on
        for from in self.pieces[self.turn][King]{
            for to in king_moves(from, self, self.turn) {
                let mv = Move::new(self, from, to, King, self.turn);
                res.push(mv);
            }
            if short_castling(self, self.turn) {
                let mv = Move::short_castling(from, self.turn);
                res.push(mv)
            }
            if long_castling(self, self.turn) {
                let mv = Move::long_castling(from, self.turn);
                res.push(mv)
            }
        }

        for piece in Piece::promotable(){
            for from in self.pieces[self.turn][piece]{
                let f = match piece {
                    Queen => queen_moves,
                    Rook => rook_moves,
                    Bishop => bishop_moves,
                    Knight => knight_moves,
                    _ => unreachable!()
                };
                for to in f(from, self, self.turn) {
                    let mv = Move::new(self, from, to, piece, self.turn);
                    res.push(mv);
                }
            }
        }

        for from in self.pieces[self.turn][Pawn]{
            let end_rank = match self.turn{
                White => 7,
                Black => 0,
            };
            for to in pawn_moves(from, self, self.turn) {
                if to / 8 == end_rank {
                    for piece in Piece::promotable() {
                        let mv = Move::promotion(self, from, to, self.turn, piece);
                        res.push(mv);
                    }
                } else {
                    let mv = Move::new(self, from, to, Pawn, self.turn);
                    res.push(mv);
                }
            }
        }
        res
    }

    /// Generates a list of all legal moves in a current position
    pub fn gen_legal_moves(&self) -> Vec<Move> {
        self.gen_moves()
            .into_iter()
            .filter(|mv| self.is_legal(mv))
            .collect::<Vec<_>>()
    }

    /// Simulates a halfmove and returns whether it is legal or not
    /// FIXME: 
    fn is_legal(&self, mv: &Move) -> bool {
        if mv.piece == King && KING_MASKS[mv.to as usize] & self.pieces[!mv.color][King] != Bitboard::empty() {
            return false
        }
        let mut cloned = self.clone();
        let res = cloned.make_move(mv);
        match res {
            Err(_) => false,
            Ok(_) => { 
                // cloned.is_check() != Some(mv.color)
                !cloned.square_is_attacked(cloned.pieces[mv.color][King].lsb_index().unwrap(), !mv.color)
            }
        }
    }

    /// Makes a move. It is assumed that the passed move is legal
    pub fn make_move(&mut self, mv: &Move) -> Result<(), ChessError> {
        if mv.piece == Pawn || mv.flag.is_castling() {
            self.repetition_history.clear();
        }
        if mv.piece == Pawn || mv.flag.is_capture() {
            self.halfmove_clock = 0;
        }
        self.en_passant = None;

        let moved_bitboard = &mut self.pieces[mv.color][mv.piece];
        match mv.flag{
            Flag::Null => (),
            Flag::Default => {
                moved_bitboard.set_0(mv.from);
                moved_bitboard.set_1(mv.to);
            },
            Flag::LongPawnMove => {
                moved_bitboard.set_0(mv.from);
                moved_bitboard.set_1(mv.to);
                if check_en_passant(mv.to, self, mv.color) {
                    self.en_passant = match mv.color {
                        White => Some(mv.to - 8),
                        Black => Some(mv.to + 8)
                    }
                };
            }
            Flag::Capture(captured) => {
                moved_bitboard.set_0(mv.from);
                moved_bitboard.set_1(mv.to);
                let captured_bitboard = &mut self.pieces[!mv.color][captured];
                captured_bitboard.set_0(mv.to);
                self.halfmove_clock = 0;
                self.repetition_history.clear();
            }
            Flag::EnPassant => {
                moved_bitboard.set_0(mv.from);
                moved_bitboard.set_1(mv.to);
                let enemy_pawns = &mut self.pieces[!mv.color][Pawn];
                match mv.color {
                    White => enemy_pawns.set_0(mv.to - 8),
                    Black => enemy_pawns.set_0(mv.to + 8),
                }
            }
            Flag::Promotion(prom) => {
                moved_bitboard.set_0(mv.from);
                let promotion_bitboard = &mut self.pieces[mv.color][prom];
                promotion_bitboard.set_1(mv.to);
            }
            Flag::CapturePromotion(captured, prom) => {
                moved_bitboard.set_0(mv.from);
                let captured_bitboard = &mut self.pieces[!mv.color][captured];
                captured_bitboard.set_0(mv.to);
                let promotion_bitboard = &mut self.pieces[mv.color][prom];
                promotion_bitboard.set_1(mv.to);
            }
            Flag::ShortCastling => {
                match mv.color {
                    White => {
                        self.pieces[White][Rook].set_0(7);
                        self.pieces[White][Rook].set_1(5);
                        self.pieces[White][King].set_0(4);
                        self.pieces[White][King].set_1(6);
                    }
                    Black => {
                        self.pieces[Black][Rook].set_0(63);
                        self.pieces[Black][Rook].set_1(61);
                        self.pieces[Black][King].set_0(60);
                        self.pieces[Black][King].set_1(62);
                    }
                }
            }
            Flag::LongCastling => {
                match mv.color {
                    White => {
                        self.pieces[White][Rook].set_0(0);
                        self.pieces[White][Rook].set_1(3);
                        self.pieces[White][King].set_0(4);
                        self.pieces[White][King].set_1(2);
                    }
                    Black => {
                        self.pieces[Black][Rook].set_0(56);
                        self.pieces[Black][Rook].set_1(59);
                        self.pieces[Black][King].set_0(60);
                        self.pieces[Black][King].set_1(58);
                    }
                }
            }
        }

        // deal with castling rights
        if mv.piece == King {
            match mv.color {
                White => {
                    self.castling_rights[White][KingSide] = false;
                    self.castling_rights[White][QueenSide] = false;
                }
                Black => {
                    self.castling_rights[Black][KingSide] = false;
                    self.castling_rights[Black][QueenSide] = false;
                }
            }
        }
        else if mv.piece == Rook {
            match mv.color {
                White => {
                    self.castling_rights[White][QueenSide] = self.castling_rights[White][QueenSide] && mv.from != 0;
                    self.castling_rights[White][KingSide] = self.castling_rights[White][KingSide] && mv.from != 7;
                }
                Black => {
                    self.castling_rights[Black][QueenSide] = self.castling_rights[Black][QueenSide] && mv.from != 56;
                    self.castling_rights[Black][KingSide] = self.castling_rights[Black][KingSide] && mv.from != 63;

                }
            }
        }

        self.update_pieces();
        self.turn = !self.turn;
        self.repetition_history.push(self.get_hash());
        
        Ok(())
    }

    /// Returns whether the provided square is attacked by the provided side
    pub fn square_is_attacked(&self, square: u8, color: Color) -> bool {
        let bitboard = Bitboard::from(square);
        // let attackers = self.pieces[color];

        if bitboard & all_pawn_captures(self.pieces[color][Pawn], color) != 0 {
            return true;
        }
        if knight_moves(square, self, !color) & self.pieces[color][Knight] != 0
            || bishop_moves(square, self, !color) & self.pieces[color][Bishop] != 0 
            || rook_moves(square, self, !color) & self.pieces[color][Rook] != 0 
            || queen_moves(square, self, !color) & self.pieces[color][Queen] != 0
            || king_moves(square, self, !color) & self.pieces[color][King] != 0 {
            return true
        }

        false
    }

    /// Returns the color of the checked side in a current position.
    /// Returns `None` if no side is in check.
    pub fn is_check(&self) -> Option<Color> {
        let white_king_square = self.pieces[White][King].lsb_index();
        let black_king_square = self.pieces[Black][King].lsb_index();
        match white_king_square {
            None => return Some(White),
            Some(val) => {
                if self.square_is_attacked(val, Black) {
                    return Some(White)
                }            
            }
        };
        match black_king_square {
            None => return Some(Black),
            Some(val) => {
                if self.square_is_attacked(val, White) {
                    return Some(Black)
                }            
            }
        }
        None
    }
}
