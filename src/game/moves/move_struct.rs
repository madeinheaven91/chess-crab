use std::fmt::Display;

use crate::{game::structs::{board::Board, color::Color, piece::Piece}, shared::functions::index_to_square};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
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
    ShortCastling,
    LongCastling,
    Null
}

impl Flag {
    pub fn is_capture(&self) -> bool {
        matches!(self, Flag::Capture(_) | Flag::CapturePromotion(_, _) | Flag::EnPassant)
    }

    pub fn is_castling(&self) -> bool {
        matches!(self, Flag::ShortCastling | Flag::LongCastling)
    }
}

impl Move {
    pub fn new(game: &Board, from: u8, to: u8, piece: Piece, color: Color) -> Self {
        let flag = match piece{
            Piece::Pawn => {
                if (to as i8 - from as i8).abs() == 16 {
                    Flag::LongPawnMove                    
                }else if to == game.en_passant.unwrap_or(0) {
                    Flag::EnPassant
                }else if game.find_piece(to).is_none() {
                    Flag::Default
                }else{
                    let (_, piece) = game.find_piece(to).unwrap();
                    Flag::Capture(piece)
                }
            },
            Piece::King => {
                if to == from + 2 {
                    Flag::ShortCastling
                }else if to + 2 == from {
                    Flag::LongCastling
                }else if game.find_piece(to).is_none() {
                    Flag::Default
                }else{
                    let (_, piece) = game.find_piece(to).unwrap();
                    Flag::Capture(piece)
                }
            },
            _ => {
                if game.find_piece(to).is_none() {
                    Flag::Default
                }else{
                    let (_, piece) = game.find_piece(to).unwrap();
                    Flag::Capture(piece)
                }
            }
        };
        // let flag = if let Some((_, captured)) = game.find_piece(to) {
        //     Flag::Capture(captured)
        // }else {
        //     match piece {
        //         Piece::Pawn => {
        //             if (to as i8 - from as i8).abs() == 16 {
        //                 Flag::LongPawnMove                    
        //             }else if to == game.en_passant.unwrap_or(0) {
        //                 Flag::EnPassant
        //             }else if game.find_piece(to).is_none() {
        //                 Flag::Default
        //             }else{
        //                 let (_, piece) = game.find_piece(to).unwrap();
        //                 Flag::Capture(piece)
        //             }
        //         },
        //         Piece::King => {
        //             if to == from + 2 {
        //                 Flag::ShortCastling
        //             }else if to + 2 == from {
        //                 Flag::LongCastling
        //             }else if game.find_piece(to).is_none() {
        //                 Flag::Default
        //             }else{
        //                 let (_, piece) = game.find_piece(to).unwrap();
        //                 Flag::Capture(piece)
        //             }
        //         },
        //         _ => Flag::Default
        //     }
        // };
        Self {
            from,
            to,
            piece,
            color,
            flag
        }
    }

    pub fn promotion(game: &Board, from: u8, to: u8, color: Color, promotion: Piece) -> Self {
        let flag = if game.find_piece(to).is_none() {
            Flag::Promotion(promotion)
        }else{
            let (_, captured) = game.find_piece(to).unwrap();
            Flag::CapturePromotion(captured, promotion)
        };
        Self {
            from,
            to,
            piece: Piece::Pawn,
            color,
            flag
        }
    }

    pub fn short_castling(king: u8, color: Color) -> Self {
        Self {
            from: king,
            to: king + 2,
            piece: Piece::King,
            color,
            flag: Flag::ShortCastling
        }
    }

    pub fn long_castling(king: u8, color: Color) -> Self {
        Self {
            from: king,
            to: king - 2,
            piece: Piece::King,
            color,
            flag: Flag::LongCastling
        }
    }
    
    pub fn null() -> Self{
        Self {
            from: 0,
            to: 0,
            piece: Piece::Pawn,
            color: Color::White,
            flag: Flag::Null
        }
    }

    pub fn algebraic(&self) -> String {
        match self.flag {
            Flag::Promotion(p) | Flag::CapturePromotion(_, p) => {
                format!("{}{}{}", index_to_square(self.from), index_to_square(self.to), p.to_string().to_lowercase())
            }
            _ => {
                format!("{}{}", index_to_square(self.from), index_to_square(self.to))
            }
        }
    }
}

impl Display for Move {
    // FIXME: Sometimes when 2 pieces can attack the same square, a starting square letter should
    // be provided. Right now it is not implemented
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.flag {
            Flag::Null => write!(f, "Null"),
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
            Flag::ShortCastling => {
                write!(f, "0-0")
            }
            Flag::LongCastling => {
                write!(f, "0-0-0")
            }
        }
    }
}
