use crate::{
    bitboard::{Bitboard, Piece},
    errors::ChessError,
    masks::{
        bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
    },
    shared::*,
};
use std::{fmt::Display, ops::Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color{
    pub fn colors() -> [Color; 2]{
        [Color::White, Color::Black]
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    start: u32,
    end: u32,
    piece: Piece,
    color: Color,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            index_to_square(self.start),
            index_to_square(self.end)
        )
    }
}

#[derive(Clone, Copy)]
pub struct Game {
    pub wk: Bitboard,
    pub wq: Bitboard,
    pub wr: Bitboard,
    pub wb: Bitboard,
    pub wn: Bitboard,
    pub wp: Bitboard,
    pub bk: Bitboard,
    pub bq: Bitboard,
    pub br: Bitboard,
    pub bb: Bitboard,
    pub bn: Bitboard,
    pub bp: Bitboard,
    pub turn: Color,
    pub white_oo: bool,
    pub white_ooo: bool,
    pub black_oo: bool,
    pub black_ooo: bool,
    pub en_passant: Option<Bitboard>,
    pub halfmoves: u8,
    pub fullmoves: u8,
}

impl Game {
    pub fn black_pieces(&self) -> Bitboard {
        Bitboard::from(
            self.bk.num()
                | self.bq.num()
                | self.br.num()
                | self.bb.num()
                | self.bn.num()
                | self.bp.num(),
        )
    }
    pub fn white_pieces(&self) -> Bitboard {
        Bitboard::from(
            self.wk.num()
                | self.wq.num()
                | self.wr.num()
                | self.wb.num()
                | self.wn.num()
                | self.wp.num(),
        )
    }
    pub fn all_pieces(&self) -> Bitboard {
        self.white_pieces() | self.black_pieces()
    }
    pub fn opposite_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.black_pieces(),
            Color::Black => self.white_pieces(),
        }
    }

    pub fn own_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    pub fn to_fen(&self) -> String {
        todo!()
    }

    pub fn get_bitboard(&self, piece: Piece, color: Color) -> Bitboard {
        use Color::*;
        use Piece::*;
        match (color, piece) {
            (White, King) => self.wk,
            (White, Queen) => self.wq,
            (White, Rook) => self.wr,
            (White, Bishop) => self.wb,
            (White, Knight) => self.wn,
            (White, Pawn) => self.wp,
            (Black, King) => self.bk,
            (Black, Queen) => self.bq,
            (Black, Rook) => self.br,
            (Black, Bishop) => self.bb,
            (Black, Knight) => self.bn,
            (Black, Pawn) => self.bp,
        }
    }

    pub fn set_bitboard(&mut self, bitboard: Bitboard, piece: Piece, color: Color) {
        use Color::*;
        use Piece::*;
        match (color, piece) {
            (White, King) => self.wk = bitboard,
            (White, Queen) => self.wq = bitboard,
            (White, Rook) => self.wr = bitboard,
            (White, Bishop) => self.wb = bitboard,
            (White, Knight) => self.wn = bitboard,
            (White, Pawn) => self.wp = bitboard,
            (Black, King) => self.bk = bitboard,
            (Black, Queen) => self.bq = bitboard,
            (Black, Rook) => self.br = bitboard,
            (Black, Bishop) => self.bb = bitboard,
            (Black, Knight) => self.bn = bitboard,
            (Black, Pawn) => self.bp = bitboard,
        }
    }

    /// Generates a list of all possible moves in a current position
    pub fn gen_moves(&self) -> Vec<Move> {
        use Color::*;
        use Piece::*;
        let mut res = vec![];
        match self.turn {
            Color::White => {
                for p in self.wk {
                    for m in king_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: King,
                            color: White,
                        });
                    }
                }
                for p in self.wq {
                    for m in queen_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Queen,
                            color: White,
                        });
                    }
                }
                for p in self.wr {
                    for m in rook_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Rook,
                            color: White,
                        });
                    }
                }
                for p in self.wb {
                    for m in bishop_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Bishop,
                            color: White,
                        });
                    }
                }
                for p in self.wn {
                    for m in knight_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Knight,
                            color: White,
                        });
                    }
                }
                for p in self.wp {
                    for m in pawn_attacks(p, *self, Color::White) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Pawn,
                            color: White,
                        });
                    }
                }
            }
            Color::Black => {
                for p in self.bk {
                    for m in king_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: King,
                            color: Black,
                        });
                    }
                }
                for p in self.bq {
                    for m in queen_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Queen,
                            color: Black,
                        });
                    }
                }
                for p in self.br {
                    for m in rook_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Rook,
                            color: Black,
                        });
                    }
                }
                for p in self.bb {
                    for m in bishop_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Bishop,
                            color: Black,
                        });
                    }
                }
                for p in self.bn {
                    for m in knight_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Knight,
                            color: Black,
                        });
                    }
                }
                for p in self.bp {
                    for m in pawn_attacks(p, *self, Color::Black) {
                        res.push(Move {
                            start: p,
                            end: m,
                            piece: Pawn,
                            color: Black,
                        });
                    }
                }
            }
        }
        res
    }

    /// Parses an algebraically notated move into `Move`.
    /// Returns `None` if the move string is invalid
    // TODO: pawn promotions (e7e8q)
    pub fn parse_move(&self, mv: &str) -> Result<Move, ChessError> {
        use Color::*;
        use Piece::*;
        if mv.len() != 4 {
            return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv)));
        };

        let (start, end) = mv.split_at(2);
        let start = square_to_index(start);
        let end = square_to_index(end);

        // Find piece
        let (piece, color) = {
            if self.wk.num() & (1 << start) != 0 {
                (King, White)
            } else if self.wq.num() & (1 << start) != 0 {
                (Queen, White)
            } else if self.wr.num() & (1 << start) != 0 {
                (Rook, White)
            } else if self.wb.num() & (1 << start) != 0 {
                (Bishop, White)
            } else if self.wn.num() & (1 << start) != 0 {
                (Knight, White)
            } else if self.wp.num() & (1 << start) != 0 {
                (Pawn, White)
            } else if self.bk.num() & (1 << start) != 0 {
                (King, Black)
            } else if self.bq.num() & (1 << start) != 0 {
                (Queen, Black)
            } else if self.br.num() & (1 << start) != 0 {
                (Rook, Black)
            } else if self.bb.num() & (1 << start) != 0 {
                (Bishop, Black)
            } else if self.bn.num() & (1 << start) != 0 {
                (Knight, Black)
            } else if self.bp.num() & (1 << start) != 0 {
                (Pawn, Black)
            } else {
                return Err(ChessError::InvalidMove(format!(
                    "No pieces can make move: {}",
                    mv
                )));
            }
        };

        Ok(Move {
            start,
            end,
            piece,
            color,
        })
    }

    pub fn make_move(&mut self, mv: &Move) {
        let mut bitboard = self.get_bitboard(mv.piece, mv.color).num();
        bitboard &= !(1u64 << mv.start);
        bitboard |= 1u64 << mv.end;
        self.set_bitboard(
            Bitboard::new(bitboard, Some(mv.piece), Some(mv.color)),
            mv.piece,
            mv.color,
        );
        for piece in Piece::pieces(){
            let piece_bitboard = self.get_bitboard(piece, !mv.color).num();
            if piece_bitboard != piece_bitboard & !(1u64 << mv.end) {
                self.set_bitboard(Bitboard::new(piece_bitboard & !(1u64 << mv.end), Some(piece), Some(!mv.color)), piece, !mv.color);
            }
        }
        self.turn = self.turn.not();
    }
}

impl Default for Game {
    fn default() -> Self {
        use Color::*;
        use Piece::*;
        Game {
            wk: Bitboard::new(WK, Some(King), Some(White)),
            wq: Bitboard::new(WQ, Some(King), Some(White)),
            wr: Bitboard::new(WR, Some(King), Some(White)),
            wb: Bitboard::new(WB, Some(King), Some(White)),
            wn: Bitboard::new(WN, Some(King), Some(White)),
            wp: Bitboard::new(WP, Some(King), Some(White)),
            bk: Bitboard::new(BK, Some(King), Some(White)),
            bq: Bitboard::new(BQ, Some(King), Some(White)),
            br: Bitboard::new(BR, Some(King), Some(White)),
            bb: Bitboard::new(BB, Some(King), Some(White)),
            bn: Bitboard::new(BN, Some(King), Some(White)),
            bp: Bitboard::new(BP, Some(King), Some(White)),
            turn: Color::White,
            white_oo: true,
            white_ooo: true,
            black_oo: true,
            black_ooo: true,
            en_passant: None,
            halfmoves: 0,
            fullmoves: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pieces: [char; 64] = [' '; 64];
        for p in self.wk {
            pieces[p as usize] = 'K';
        }
        for p in self.wq {
            pieces[p as usize] = 'Q';
        }
        for p in self.wr {
            pieces[p as usize] = 'R';
        }
        for p in self.wb {
            pieces[p as usize] = 'B';
        }
        for p in self.wn {
            pieces[p as usize] = 'N';
        }
        for p in self.wp {
            pieces[p as usize] = 'P';
        }
        for p in self.bk {
            pieces[p as usize] = 'k';
        }
        for p in self.bq {
            pieces[p as usize] = 'q';
        }
        for p in self.br {
            pieces[p as usize] = 'r';
        }
        for p in self.bb {
            pieces[p as usize] = 'b';
        }
        for p in self.bn {
            pieces[p as usize] = 'n';
        }
        for p in self.bp {
            pieces[p as usize] = 'p';
        }
        for i in (0..8).rev() {
            writeln!(f, "+---+---+---+---+---+---+---+---+")?;
            for j in 0..8 {
                write!(f, "| {} ", pieces[i * 8 + j])?;
            }
            writeln!(f, "| {}", i + 1)?;
        }
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        writeln!(f, "  a   b   c   d   e   f   g   h  ")?;
        writeln!(f, "\nMove: {:?}", self.turn)?;
        Ok(())
    }
}
