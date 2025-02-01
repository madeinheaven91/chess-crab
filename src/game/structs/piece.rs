use std::{fmt::Display, ops::{Index, IndexMut}};

use super::{bitboard::Bitboard, color::Color};


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece{
    pub fn pieces() -> [Piece; 6]{
        [Piece::King, Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight, Piece::Pawn]
    }
    pub fn promotable() -> [Piece; 4]{
        [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight]
    }
    pub fn char(&self) -> char {
        match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Bishop => 'B',
            Self::Knight => 'N',
            Self::Pawn => 'P'
        }    
    }

    pub fn symbol(piece: Piece, color: Color) -> char {
        let symbols = ['♚', '♛', '♜', '♝', '♞', '♟', '♔', '♕', '♖', '♗', '♘', '♙'];
        let index = color as usize * 6 + piece as usize;
        symbols[index]
    }
}

impl Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::King => "K",
            Self::Queen => "Q",
            Self::Bishop => "B",
            Self::Knight => "N",
            Self::Rook => "R",
            Self::Pawn => ""
        };
        write!(f, "{}", char)
    }
}

impl From<Piece> for usize{
    fn from(value: Piece) -> Self {
        match value {
            Piece::King => 0,
            Piece::Queen => 1,
            Piece::Rook => 2,
            Piece::Bishop => 3,
            Piece::Knight => 4,
            Piece::Pawn => 5
        }
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::King,
            1 => Piece::Queen,
            2 => Piece::Rook,
            3 => Piece::Bishop,
            4 => Piece::Knight,
            _ => Piece::Pawn
        }
    }
}

impl Index<Piece> for [Bitboard; 6]{
    type Output = Bitboard;
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Piece> for [Bitboard; 6]{
    fn index_mut(&mut self, index: Piece) -> &mut Bitboard {
        &mut self[index as usize]
    }
}
