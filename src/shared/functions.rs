use super::consts::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8};
use crate::{game::structs::bitboard::Bitboard, shared::errors::ChessError};

pub fn square_to_index(square: &str) -> Result<u32, ChessError>{
    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];

    let file = files
        .iter()
        .position(|&x| x == square.chars().next().unwrap());
    let file = match file {
        None => return Err(ChessError::SquareParseError(square.to_string())),
        Some(f) => f as u32
    };

    let rank = ranks
        .iter()
        .position(|&x| x == square.chars().nth(1).unwrap());
    let rank = match rank {
        None => return Err(ChessError::SquareParseError(square.to_string())),
        Some(r) => r as u32
    };
    
    Ok(rank * 8 + file)
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

pub fn lsb_index(bitboard: Bitboard) -> Option<u32> {
    match bitboard.num() {
        0 => None,
        _ => Some(bitboard.num().trailing_zeros()),
    }
}

pub fn msb_index(bitboard: Bitboard) -> Option<u32> {
    match bitboard.num() {
        0 => None,
        _ => Some(63 - bitboard.num().leading_zeros()),
    }
}

pub fn lsb(bitboard: Bitboard) -> Option<Bitboard> {
    let index = lsb_index(bitboard)?;
    Some(Bitboard::from(index))
}

pub fn msb(bitboard: Bitboard) -> Option<Bitboard> {
    let index = msb_index(bitboard)?;
    Some(Bitboard::from(index))
    
    // 1u64.checked_shl(index))
}
