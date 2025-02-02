use std::{fmt::Display, ops::{Index, IndexMut, Not}};

use super::bitboard::Bitboard;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn colors() -> [Color; 2] {
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

impl From<Color> for usize{
    fn from(value: Color) -> Self {
        match value {
            Color::White => 0,
            Color::Black => 1
        }
    }
}

impl Index<Color> for [[Bitboard; 6]; 2]{
    type Output = [Bitboard; 6];
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Color> for [[Bitboard; 6]; 2]{
    fn index_mut(&mut self, index: Color) -> &mut [Bitboard; 6] {
        &mut self[index as usize]
    }
}

impl Index<Color> for [[bool; 2]; 2]{
    type Output = [bool; 2];
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Color> for [[bool; 2]; 2]{
    fn index_mut(&mut self, index: Color) -> &mut [bool; 2] {
        &mut self[index as usize]
    }
}

pub enum Castling {
    KingSide,
    QueenSide
}

impl Index<Castling> for [bool; 2] {
    type Output = bool;
    fn index(&self, index: Castling) -> &Self::Output {
        match index {
            Castling::KingSide => &self[0],
            Castling::QueenSide => &self[1]
        }
    }
}

impl IndexMut<Castling> for [bool; 2] {
    fn index_mut(&mut self, index: Castling) -> &mut bool {
        match index {
            Castling::KingSide => &mut self[0],
            Castling::QueenSide => &mut self[1]
        }
    }
}
