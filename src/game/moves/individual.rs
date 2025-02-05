use crate::{
    game::structs::{
        bitboard::Bitboard,
        board::Board,
        color::{Castling, Color},
        piece::Piece,
    },
    shared::{
        consts::{DIRECTION, FILE_A, FILE_H, RANK_2, RANK_7},
        functions::{lsb_index, msb_index},
        statics::{
            masks::{KING_MASKS, KNIGHT_MASKS, PAWN_CAPTURE_MASKS},
            rays::{INCL_RAY, RAY},
        },
    },
};

use Castling::*;
use Color::*;

/// Returns a bitboard of pseudolegal pawn advances of a given color
pub fn pawn_advances(pawn: u32, game: &Board, color: Color) -> Bitboard {
    let pawn = 1u64 << pawn;
    let blockers = game.all_pieces.num() & !pawn;
    let advances = match color {
        Color::White => (((pawn & RANK_2) << 16) & !(blockers << 8)) | (pawn << 8),
        Color::Black => (((pawn & RANK_7) >> 16) & !(blockers >> 8)) | (pawn >> 8),
    } & !blockers;
    Bitboard::from(advances)
}

/// Returns a bitboard of pseudolegal pawn captures of a given color
pub fn pawn_captures(pawn: u32, game: &Board, color: Color) -> Bitboard {
    let enemies = game.enemies(color);
    let captures = PAWN_CAPTURE_MASKS[color as usize][pawn as usize];
    let en_passant = match game.en_passant {
        Some(en_passant) => Bitboard::from(en_passant),
        None => Bitboard::empty(),
    };
    captures & (enemies | en_passant)
}

/// Returns a bitboard of pseudolegal pawn moves (advances and captures) of a given color
pub fn pawn_moves(pawn: u32, game: &Board, color: Color) -> Bitboard {
    pawn_advances(pawn, game, color) | pawn_captures(pawn, game, color)
}

/// Returns a mask of all capture squares for pawns of a given color
pub fn all_pawn_captures(pawns: u64, color: Color) -> Bitboard {
    let captures = match color {
        Color::White => ((pawns << 9) & !FILE_A) | ((pawns << 7) & !FILE_H),
        Color::Black => ((pawns >> 9) & !FILE_H) | ((pawns >> 7) & !FILE_A),
    };
    Bitboard::from(captures)
}

/// Returns a bitboard of pseudolegal king moves of a given color
pub fn king_moves(king: u32, game: &Board, color: Color) -> Bitboard {
    let blockers = game.friends(color);
    let mask = KING_MASKS[king as usize];
    mask & !blockers
}

pub fn short_castling(king: u32, game: &Board, color: Color) -> Bitboard {
    let king_num = 1u64 << king;

    let right = match color {
        Color::White => game.castling_rights[White][KingSide],
        Color::Black => game.castling_rights[Black][KingSide],
    };
    if !right {
        return Bitboard::empty();
    }
    let blockers = game.all_pieces;
    let squares_are_empty = ((king_num << 1 | king_num << 2) & blockers.num()) == 0;
    let squares_are_attacked =
        game.square_is_attacked(king + 1, !color) || game.square_is_attacked(king + 2, !color);

    if game.is_check().is_none() && squares_are_empty && !squares_are_attacked {
        Bitboard::from(king_num << 2)
    } else {
        Bitboard::empty()
    }
}

pub fn long_castling(king: u32, game: &Board, color: Color) -> Bitboard {
    let king_num = 1u64 << king;

    let right = match color {
        Color::White => game.castling_rights[White][QueenSide],
        Color::Black => game.castling_rights[Black][QueenSide],
    };
    if !right {
        return Bitboard::empty();
    }
    let blockers = game.all_pieces;
    let squares_are_empty = ((king_num >> 1 | king_num >> 2 | king_num >> 3) & blockers.num()) == 0;
    let squares_are_attacked = game.square_is_attacked(king - 1, !color)
        || game.square_is_attacked(king - 2, !color)
        || game.square_is_attacked(king - 3, !color);

    if game.is_check().is_none() && squares_are_empty && !squares_are_attacked {
        Bitboard::from(king_num >> 2)
    } else {
        Bitboard::empty()
    }
}

/// Returns a bitboard of pseudolegal knight moves of a given color
pub fn knight_moves(knight: u32, game: &Board, color: Color) -> Bitboard {
    let blockers = game.friends(color);
    let mask = KNIGHT_MASKS[knight as usize];
    mask & !blockers
}

/// Returns a bitboard of pseudolegal rook moves of a given color
pub fn rook_moves(index: u32, game: &Board, color: Color) -> Bitboard {
    use DIRECTION::*;
    let e = scan_ray(index, game, color, E);
    let w = scan_ray(index, game, color, W);
    let n = scan_ray(index, game, color, N);
    let s = scan_ray(index, game, color, S);

    e | w | s | n
}

pub fn bishop_moves(index: u32, game: &Board, color: Color) -> Bitboard {
    use DIRECTION::*;
    let ne = scan_ray(index, game, color, NE);
    let nw = scan_ray(index, game, color, NW);
    let se = scan_ray(index, game, color, SE);
    let sw = scan_ray(index, game, color, SW);

    ne | nw | se | sw
}

pub fn queen_moves(queen: u32, game: &Board, color: Color) -> Bitboard {
    bishop_moves(queen, game, color) | rook_moves(queen, game, color)
}

fn scan_ray(index: u32, game: &Board, color: Color, direction: DIRECTION) -> Bitboard {
    let friends = game.friends(color);
    let enemies = game.enemies(color);

    let blockers = match direction {
        DIRECTION::E => friends | (enemies << 1),
        DIRECTION::W => friends | (enemies >> 1),
        DIRECTION::N => friends | (enemies << 8),
        DIRECTION::S => friends | (enemies >> 8),
        DIRECTION::NE => friends | (enemies << 9),
        DIRECTION::SW => friends | (enemies >> 9),
        DIRECTION::NW => friends | (enemies << 7),
        DIRECTION::SE => friends | (enemies >> 7),
    };
    let ray = RAY[index as usize][direction as usize];
    let ray_blocker = match direction {
        DIRECTION::W | DIRECTION::S | DIRECTION::SE | DIRECTION::SW => msb_index(blockers & ray),
        _ => lsb_index(blockers & ray),
    };
    match ray_blocker {
        Some(ray_blocker) => {
            ray & !Bitboard::from(1u64) & !INCL_RAY[ray_blocker as usize][direction as usize]
        }
        None => ray,
    }
}

pub fn check_en_passant(pawn: u32, game: &Board, color: Color) -> bool {
    let enemy_pawns = game.pieces[!color][Piece::Pawn].num();
    let pawn = 1u64 << pawn;
    (enemy_pawns & (pawn << 1)) != 0 || (enemy_pawns & (pawn >> 1)) != 0
}
