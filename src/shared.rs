use lazy_static::lazy_static;

use crate::bitboard::Bitboard;

// RANKS
pub const RANK_8: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;
pub const RANK_7: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
pub const RANK_6: u64 = 0b0000000000000000111111110000000000000000000000000000000000000000;
pub const RANK_5: u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
pub const RANK_4: u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
pub const RANK_3: u64 = 0b0000000000000000000000000000000000000000111111110000000000000000;
pub const RANK_2: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
pub const RANK_1: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;

// FILES
pub const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
pub const FILE_G: u64 = 0b0100000001000000010000000100000001000000010000000100000001000000;
pub const FILE_F: u64 = 0b0010000000100000001000000010000000100000001000000010000000100000;
pub const FILE_E: u64 = 0b0001000000010000000100000001000000010000000100000001000000010000;
pub const FILE_D: u64 = 0b0000100000001000000010000000100000001000000010000000100000001000;
pub const FILE_C: u64 = 0b0000010000000100000001000000010000000100000001000000010000000100;
pub const FILE_B: u64 = 0b0000001000000010000000100000001000000010000000100000001000000010;
pub const FILE_A: u64 = 0b0000000100000001000000010000000100000001000000010000000100000001;

// DEFAULT PIECES

pub const WK: u64 = 0b0000000000000000000000000000000000000000000000000000000000010000;
pub const WQ: u64 = 0b0000000000000000000000000000000000000000000000000000000000001000;
pub const WR: u64 = 0b0000000000000000000000000000000000000000000000000000000010000001;
pub const WB: u64 = 0b0000000000000000000000000000000000000000000000000000000000100100;
pub const WN: u64 = 0b0000000000000000000000000000000000000000000000000000000001000010;
pub const WP: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
pub const BK: u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;
pub const BQ: u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
pub const BR: u64 = 0b1000000100000000000000000000000000000000000000000000000000000000;
pub const BB: u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;
pub const BN: u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;
pub const BP: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;

// RAY
lazy_static! {
    pub static ref RAY: [[u64; 8]; 64] = {
        let mut res = [[0; 8]; 64];
        (0..64).for_each(|i| {
            (0..8).for_each(|j| {
                let direction = DIRECTION::from(j);
                res[i][j] = gen_ray(1u64 << i, direction);
            });
        });
        res
    };
}

pub enum DIRECTION {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<usize> for DIRECTION {
    fn from(val: usize) -> Self {
        match val {
            0 => DIRECTION::N,
            1 => DIRECTION::NE,
            2 => DIRECTION::E,
            3 => DIRECTION::SE,
            4 => DIRECTION::S,
            5 => DIRECTION::SW,
            6 => DIRECTION::W,
            _ => DIRECTION::NW,
        }
    }
}

impl From<DIRECTION> for usize {
    fn from(value: DIRECTION) -> Self {
        match value{
            DIRECTION::N => 0,
            DIRECTION::NE => 1,
            DIRECTION::E => 2,
            DIRECTION::SE => 3,
            DIRECTION::S => 4,
            DIRECTION::SW => 5,
            DIRECTION::W => 6,
            DIRECTION::NW => 7,
        }
    }
}

pub fn col(square: u64) -> u64 {
    match (square.trailing_zeros()) % 8 {
        7 => FILE_A,
        6 => FILE_B,
        5 => FILE_C,
        4 => FILE_D,
        3 => FILE_E,
        2 => FILE_F,
        1 => FILE_G,
        _ => FILE_H,
    }
}

pub fn row(square: u64) -> u64 {
    match (square.trailing_zeros()) / 8 {
        7 => RANK_8,
        6 => RANK_7,
        5 => RANK_6,
        4 => RANK_5,
        3 => RANK_4,
        2 => RANK_3,
        1 => RANK_2,
        _ => RANK_1,
    }
}

// FIXME: find better solutions
// if square param contains more than 1 significant bit, function works incorrectly
// make rays static
pub fn gen_ray(square: u64, direction: DIRECTION) -> u64 {
    use DIRECTION::*;
    if square == 0 {
        return 0;
    };
    match direction {
        // TODO: not optimal solution, find a better one
        N => {
            square << 8
                | square << 16
                | square << 24
                | square << 32
                | square << 40
                | square << 48
                | square << 56
        }
        S => {
            square >> 8
                | square >> 16
                | square >> 24
                | square >> 32
                | square >> 40
                | square >> 48
                | square >> 56
        }
        // TODO: loops are slow, need to find a better way
        E => {
            let mut res = square;
            let rank = Bitboard::lsb_index(square).unwrap() / 8;
            for i in 0..8 {
                let new_bit = square << i;
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() / 8 != rank {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        W => {
            let mut res = square;
            let rank = Bitboard::lsb_index(square).unwrap() / 8;
            for i in 0..8 {
                let new_bit = square >> i;
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() / 8 != rank {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        NE => {
            let mut res = square;
            for i in 0..8 {
                let new_bit = square << (i * 9);
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() % 8 == 0 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        SW => {
            let mut res = square;
            for i in 0..8 {
                let new_bit = square >> (i * 9);
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() % 8 == 7 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        SE => {
            let mut res = square;
            for i in 0..8 {
                let new_bit = square >> (i * 7);
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() % 8 == 0 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        NW => {
            let mut res = square;
            for i in 0..8 {
                let new_bit = square << (i * 7);
                if new_bit == 0 {
                    break;
                }
                if Bitboard::lsb_index(new_bit).unwrap() % 8 == 7 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
    }
}

pub fn square_to_index(square: &str) -> u32 {
    if square.len() != 2 {
        panic!("Incorrect square")
    };
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
    let rank = ranks
        .iter()
        .position(|&x| x == square.chars().nth(1).unwrap())
        .unwrap() as u32;
    let letter = letters
        .iter()
        .position(|&x| x == square.chars().next().unwrap())
        .unwrap() as u32;
    rank * 8 + letter
}

pub fn index_to_square(index: u32) -> String {
    if !(0..64).contains(&index) {
        panic!()
    };
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
    let rank = ranks[(index / 8) as usize];
    let letter = letters[(index % 8) as usize];
    format!("{}{}", letter, rank)
}
