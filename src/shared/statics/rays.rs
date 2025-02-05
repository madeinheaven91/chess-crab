use crate::game::structs::bitboard::Bitboard;

use crate::shared::consts::DIRECTION;
use crate::shared::functions::lsb_index;
use lazy_static::lazy_static;

// RAY
lazy_static! {
    /// RAY[square][direction]
    pub static ref RAY: [[Bitboard; 8]; 64] = {
        let mut res = [[Bitboard::empty(); 8]; 64];
        (0..64).for_each(|i| {
            (0..8).for_each(|j| {
                let direction = DIRECTION::from(j);
                res[i][j] = gen_ray(i as u32, direction);
            });
        });
        res
    };

    pub static ref INCL_RAY: [[Bitboard; 8]; 64] = {
        let mut res = [[Bitboard::empty(); 8]; 64];
        (0..64).for_each(|i| {
            (0..8).for_each(|j| {
                let direction = DIRECTION::from(j);
                res[i][j] = gen_incl_ray(i as u32, direction);
            });
        });
        res
    };
}


fn gen_ray(square: u32, direction: DIRECTION) -> Bitboard {
    use DIRECTION::*;
    if !(0..64).contains(&square) {
        return Bitboard::empty();
    }
    let square = Bitboard::from(square);
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

fn gen_incl_ray(square: u32, direction: DIRECTION) -> Bitboard {
    let ray = gen_ray(square, direction);
    ray | Bitboard::from(square)
}
