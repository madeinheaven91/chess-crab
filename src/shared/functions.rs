use super::consts::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8};

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
    let index = lsb_index(bitboard)?;
    Some(1u64 << index)
}
//
pub fn msb(bitboard: u64) -> Option<u64> {
    let index = msb_index(bitboard)?;
    1u64.checked_shl(index)
}
