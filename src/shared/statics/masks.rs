use lazy_static::lazy_static;

use crate::game::structs::bitboard::Bitboard;

use super::consts::{FILE_A, FILE_B, FILE_G, FILE_H};

lazy_static! {
    pub static ref KNIGHT_MASKS: [Bitboard; 64] = gen_knight_masks();
    pub static ref KING_MASKS: [Bitboard; 64] = gen_king_masks();
    // pub static ref PAWN_ADVANCES_MASKS: [[Bitboard; 64]; 2] = gen_pawn_advances_masks();
    pub static ref PAWN_CAPTURE_MASKS: [[Bitboard; 64]; 2] = gen_pawn_capture_masks();
}

fn gen_knight_masks() -> [Bitboard; 64] {
    let mut masks: [Bitboard; 64] = [Bitboard::empty(); 64];
    (0..64).for_each(|i| {
        let knight = Bitboard::from(i as u8);
        masks[i] = 
            ((knight >> 17 | knight << 15) & !*FILE_H) 
            | ((knight >> 15 | knight << 17) & !*FILE_A) 
            | ((knight >> 10 | knight << 6) & !*FILE_H & !*FILE_G) | ((knight >> 6 | knight << 10) & !*FILE_A & !*FILE_B)
    });
    masks
}

fn gen_king_masks() -> [Bitboard; 64] {
    let mut masks: [Bitboard; 64] = [Bitboard::empty(); 64];
    (0..64).for_each(|i| {
        let king = Bitboard::from(i as u8);
        masks[i] = 
            king >> 8
            | king << 8
            | ((king >> 1 | king >> 9 | king << 7) & !*FILE_H) | ((king >> 7 | king << 1 | king << 9) & !*FILE_A)
    });
    masks
}

// fn gen_pawn_advances_masks() -> [[Bitboard; 64]; 2] {
//         Color::White => (((pawn & RANK_2) << 16) & !(blockers << 8)) | (pawn << 8),
//         Color::Black => (((pawn & RANK_7) >> 16) & !(blockers >> 8)) | (pawn >> 8),
//
// }

fn gen_pawn_capture_masks() -> [[Bitboard; 64]; 2] {
    let mut masks: [[Bitboard; 64]; 2] = [[Bitboard::empty(); 64]; 2];
    for i in 0..64 {
        let pawn = Bitboard::from(i as u8);
        masks[0][i] = ((pawn << 9) & !*FILE_A) | ((pawn << 7) & !*FILE_H);
        masks[1][i] = ((pawn >> 9) & !*FILE_H) | ((pawn >> 7) & !*FILE_A);
    }
    masks
}
