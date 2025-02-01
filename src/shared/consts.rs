use lazy_static::lazy_static;
use super::functions::lsb_index;

// INDICES
// pub const PAWN: usize = 0;
// pub const KNIGHT: usize = 1;
// pub const BISHOP: usize = 2;
// pub const ROOK: usize = 3;
// pub const QUEEN: usize = 4;
// pub const KING: usize = 5;
//
// pub const WHITE: usize = 0;
// pub const BLACK: usize = 1;

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
    /// RAY[square][direction]
    pub static ref RAY: [[u64; 8]; 64] = {
        let mut res = [[0; 8]; 64];
        (0..64).for_each(|i| {
            (0..8).for_each(|j| {
                let direction = DIRECTION::from(j);
                res[i][j] = gen_ray(i as u32, direction);
            });
        });
        res
    };

    pub static ref INCL_RAY: [[u64; 8]; 64] = {
        let mut res = [[0; 8]; 64];
        (0..64).for_each(|i| {
            (0..8).for_each(|j| {
                let direction = DIRECTION::from(j);
                res[i][j] = gen_incl_ray(i as u32, direction);
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


// FIXME: find better solutions
// if square param contains more than 1 significant bit, function works incorrectly
fn gen_ray(square: u32, direction: DIRECTION) -> u64 {
    use DIRECTION::*;
    if !(0..64).contains(&square) {
        return 0;
    }
    let square = 1u64 << square;
    match direction {
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
        E => {
            let mut res = square;
            let rank = lsb_index(square).unwrap() / 8;
            for i in 0..8 {
                let new_bit = square << i;
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() / 8 != rank {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        W => {
            let mut res = square;
            let rank = lsb_index(square).unwrap() / 8;
            for i in 0..8 {
                let new_bit = square >> i;
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() / 8 != rank {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        NW => {
            let mut res = square;
            for i in 1..8 {
                let new_bit = square << (i * 7);
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() % 8 == 7 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        SW => {
            let mut res = square;
            for i in 1..8 {
                let new_bit = square >> (i * 9);
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() % 8 == 7 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        SE => {
            let mut res = square;
            for i in 1..8 {
                let new_bit = square >> (i * 7);
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() % 8 == 0 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
        NE => {
            let mut res = square;
            for i in 1..8 {
                let new_bit = square << (i * 9);
                if new_bit == 0 {
                    break;
                }
                if lsb_index(new_bit).unwrap() % 8 == 0 {
                    break;
                }
                res |= new_bit;
            }
            res & !square
        }
    }
}


fn gen_incl_ray(square: u32, direction: DIRECTION) -> u64 {
    let ray = gen_ray(square, direction);
    ray | (1u64 << square)
}
