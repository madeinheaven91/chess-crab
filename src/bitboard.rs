use std::{
    fmt::{Binary, Display},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

use crate::game::Color;

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
}

/// A wrapper type for u64 with chess util methods.
/// Mapped in a Little-Endian Rank-File style
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitboard {
    num: u64,
    kind: Option<Piece>,
    color: Option<Color>
}

impl Bitboard {
    pub fn new(num: u64, kind: Option<Piece>, color: Option<Color>) -> Self {
        Bitboard {
            num,
            kind,
            color
        }
    }

    pub fn num(&self) -> u64 {
        self.num
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn kind(&self) -> Option<Piece> {
        self.kind
    }

    pub fn get_square(&self, index: u32) -> u32 {
        ((self.num >> index) % 2) as u32
    }

    pub fn lsb_index(bitboard: u64) -> Option<u32> {
        match bitboard {
            0 => None,
            _ => Some(bitboard.trailing_zeros()),
        }
    }

    pub fn msb_index(bitboard: u64) -> Option<u32> {
        match bitboard {
            0 => None,
            _ => Some(63 - bitboard.leading_zeros()),
        }
    }

    pub fn lsb(bitboard: u64) -> Option<u64> {
        let index = Self::lsb_index(bitboard)?;
        Some(1u64 << index)
    }
    //
    pub fn msb(bitboard: u64) -> Option<u64> {
        let index = Self::msb_index(bitboard)?;
        1u64.checked_shl(index)
    }
}

// Bitboards are mapped in a Little-Endian Rank-File style
//
//  +----+----+----+----+----+----+----+----+
//  | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 8
//  +----+----+----+----+----+----+----+----+
//  | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 7
//  +----+----+----+----+----+----+----+----+
//  | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 6
//  +----+----+----+----+----+----+----+----+
//  | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 5
//  +----+----+----+----+----+----+----+----+
//  | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 4
//  +----+----+----+----+----+----+----+----+
//  | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 3
//  +----+----+----+----+----+----+----+----+
//  |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 2
//  +----+----+----+----+----+----+----+----+
//  |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 | 1
//  +----+----+----+----+----+----+----+----+
//     a    b    c    d    e    f    g    h
//
// Get bit index by square label: file + (rank - 1) * 8
//
//
// In this mapping, single step increments are as follows:
// left shift is positive (<<)
// right shift is negative (>>)
//
// northwest    north   northeast
// noWe         nort         noEa
//         +7    +8    +9
//             \  |  /
// west    -1 <-  0 -> +1    east
//             /  |  \
//         -9    -8    -7
// soWe         sout         soEa
// southwest    south   southeast
//
// squareIndexBigEndianFile    = squareIndexLittleEndianFile ^ 7;
// squareIndexLittleEndianFile = squareIndexBigEndianFile    ^ 7;
// squareIndexBigEndianRank    = squareIndexLittleEndianRank ^ 56;
// squareIndexLittleEndianRank = squareIndexBigEndianRank    ^ 56;
//
// squareIndex {0..63} = 8*rankIndex + fileIndex;
// rankIndex   {0..7}  = squareIndex div 8; // squareIndex >> 3;
// fileIndex   {0..7}  = squareIndex mod 8; // squareIndex & 7;

impl Binary for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = (0..8)
            .rev()
            .map(|r| {
                (0..8).fold(String::new(), |acc, c| {
                    if (self.num >> (r * 8 + c)) & 1 == 1 {
                        acc + "o"
                    } else {
                        acc + "."
                    }
                }) + "\n"
            })
            .collect::<String>();
        write!(f, "{}", board)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pieces: [char; 64] = [' '; 64];
        for p in self.into_iter() {
            pieces[p as usize] = 'O';
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
        Ok(())
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard {
            num: value,
            kind: None,
            color: None
        }
    }
}

/// Creates a bitboard from a bit index
impl From<u32> for Bitboard {
    fn from(value: u32) -> Self {
        Bitboard {
            num: 1 << value,
            kind: None,
            color: None
        }
    }
}


/// Initialize a bitboard from an array that resembles an actual board
/// e.g.
/// `Bitboard::from([
///     0b00000000,
///     0b00000000,
///     0b00000000,
///     0b00100000,
///     0b00001000,
///     0b01000100,
///     0b00100011,
///     0b00000000,
/// ])`
impl From<[u32; 8]> for Bitboard {
    fn from(value: [u32; 8]) -> Self {
        Bitboard {
            // num: value.iter().fold(0, |acc, x| (acc << 8) | *x as u64),
            num: value.iter().map(|x| reverse(*x)).fold(0, |acc, x| (acc << 8) | x as u64),
            kind: None,
            color: None
        }
    }
}
fn reverse(num: u32) -> u32{
  ((num & 0x01) << 7)
| ((num & 0x02) << 5)
| ((num & 0x04) << 3)
| ((num & 0x08) << 1)
| ((num & 0x10) >> 1)
| ((num & 0x20) >> 3)
| ((num & 0x40) >> 5)
| ((num & 0x80) >> 7)
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard{
            num: self.num | rhs.num,
            kind: self.kind,
            color: self.color
        }
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard{
            num: self.num & rhs.num,
            kind: self.kind,
            color: self.color
        }
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard{ 
            num: self.num ^ rhs.num,
            kind: self.kind,
            color: self.color 
        }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.num |= rhs.num;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.num &= rhs.num;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.num ^= rhs.num;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard{
            num: !self.num,
            kind: self.kind,
            color: self.color
        }
    }
}

impl Shl<u32> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        Bitboard{
            num: self.num << rhs,
            kind: self.kind,
            color: self.color
        }
    }
}

impl ShlAssign<u32> for Bitboard {
    fn shl_assign(&mut self, rhs: u32) {
        self.num <<= rhs;
    }
}

impl Shr<u32> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        Bitboard{
            num: self.num >> rhs,
            kind: self.kind,
            color: self.color
        }
    }
}

impl ShrAssign<u32> for Bitboard {
    fn shr_assign(&mut self, rhs: u32) {
        self.num >>= rhs;
    }
}

pub struct BitboardIterator {
    num: u64,
}

impl From<Bitboard> for BitboardIterator {
    fn from(val: Bitboard) -> Self {
        BitboardIterator { num: val.num }
    }
}

/// Iterates over the bits of a bitboard. Returns indices of the 1 bits.
impl Iterator for BitboardIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..=63 {
            let bit = (self.num >> i) % 2;
            if bit == 1 {
                self.num &= !(1 << i);
                return Some(i);
            }
        }
        None
    }
}

impl IntoIterator for Bitboard {
    type Item = u32;
    type IntoIter = BitboardIterator;
    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator::from(self)
    }
}
