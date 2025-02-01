use std::fmt::Display;

use crate::shared::functions::index_to_square;

use super::{color::Color, game::Game, piece::Piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u32,
    pub to: u32,
    pub piece: Piece,
    pub color: Color,
    pub flag: Flag,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag {
    Default,
    Capture(Piece),
    Promotion(Piece),
    CapturePromotion(Piece, Piece),
    LongPawnMove,
    EnPassant,
    ShortCastle,
    LongCastle,
}

impl Move {
    pub fn new(game: &Game, from: u32, to: u32, piece: Piece, color: Color) -> Self {
        let flag = match piece{
            Piece::Pawn => {
                if (to as i8 - from as i8).abs() == 16 {
                    Flag::LongPawnMove                    
                }else if to == game.en_passant.unwrap_or(0) {
                    Flag::EnPassant
                }else if game.find_piece(!color, to).is_none() {
                    Flag::Default
                }else{
                    let piece = game.find_piece(!color, to).unwrap();
                    Flag::Capture(piece)
                }
            },
            // Piece::King => todo!(),
            _ => {
                if game.find_piece(!color, to).is_none() {
                    Flag::Default
                }else{
                    let piece = game.find_piece(!color, to).unwrap();
                    Flag::Capture(piece)
                }
            }
        };
        Self {
            from,
            to,
            piece,
            color,
            flag
        }
    }
    pub fn promotion(game: &Game, from: u32, to: u32, piece: Piece, color: Color, promotion: Piece) -> Self {
        let flag = if game.find_piece(!color, to).is_none() {
            Flag::Promotion(promotion)
        }else{
            let captured = game.find_piece(!color, to).unwrap();
            Flag::CapturePromotion(captured, promotion)
        };
        Self {
            from,
            to,
            piece,
            color,
            flag
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.flag {
            Flag::Default | Flag::LongPawnMove => match self.piece {
                Piece::Pawn => {
                    write!(f, "{}", index_to_square(self.to))
                }
                _ => {
                    write!(f, "{}{}", self.piece, index_to_square(self.to))
                }
            },
            Flag::Capture(_) | Flag::EnPassant => match self.piece {
                Piece::Pawn => {
                    write!(
                        f,
                        "{}x{}",
                        index_to_square(self.from).split_at(1).0,
                        index_to_square(self.to)
                    )
                }
                _ => {
                    write!(f, "{}x{}", self.piece, index_to_square(self.to))
                }
            },
            Flag::Promotion(prom) => {
                write!(f, "{}={}", index_to_square(self.to), prom)
            }
            Flag::CapturePromotion(_, prom) => {
                write!(
                    f,
                    "{}x{}={}",
                    index_to_square(self.from).split_at(1).0,
                    index_to_square(self.to),
                    prom
                )
            }
            Flag::ShortCastle => {
                write!(f, "0-0")
            }
            Flag::LongCastle => {
                write!(f, "0-0-0")
            }
        }
    }
}
