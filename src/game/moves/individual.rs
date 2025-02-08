use crate::{
    game::structs::{
        bitboard::Bitboard,
        board::Board,
        color::{Castling, Color},
        piece::Piece,
    },
    shared::{
        statics::{consts::{FILE_A, FILE_H, RANK_2, RANK_7}, masks::{KING_MASKS, KNIGHT_MASKS, PAWN_CAPTURE_MASKS}, rays::{INCL_RAY, RAY}},
        structs::DIRECTION,
    },
};

use Castling::*;

/// Returns a bitboard of pseudolegal pawn advances of a given color
pub fn pawn_advances(pawn: u8, game: &Board, color: Color) -> Bitboard {
    let pawn = Bitboard::from(pawn);
    let blockers = game.all_pieces & !pawn;
    (match color {
        Color::White => (((pawn & *RANK_2) << 16) & !(blockers << 8)) | (pawn << 8),
        Color::Black => (((pawn & *RANK_7) >> 16) & !(blockers >> 8)) | (pawn >> 8),
    } & !blockers)
}

/// Returns a bitboard of pseudolegal pawn captures of a given color
pub fn pawn_captures(pawn: u8, game: &Board, color: Color) -> Bitboard {
    let enemies = game.enemies(color);
    let captures = PAWN_CAPTURE_MASKS[color as usize][pawn as usize];
    let en_passant = match game.en_passant {
        Some(en_passant) => Bitboard::from(en_passant),
        None => Bitboard::empty(),
    };
    captures & (enemies | en_passant)
}

/// Returns a bitboard of pseudolegal pawn moves (advances and captures) of a given color
pub fn pawn_moves(pawn: u8, game: &Board, color: Color) -> Bitboard {
    pawn_advances(pawn, game, color) | pawn_captures(pawn, game, color)
}

/// Returns a mask of all capture squares for pawns of a given color
pub fn all_pawn_captures(pawns: Bitboard, color: Color) -> Bitboard {
    match color {
        Color::White => ((pawns << 9) & !*FILE_A) | ((pawns << 7) & !*FILE_H),
        Color::Black => ((pawns >> 9) & !*FILE_H) | ((pawns >> 7) & !*FILE_A),
    }
}

/// Returns a bitboard of pseudolegal king moves of a given color
pub fn king_moves(king: u8, game: &Board, color: Color) -> Bitboard {
    let blockers = game.friends(color);
    let mask = KING_MASKS[king as usize];
    mask & !blockers
}

pub fn short_castling(game: &Board, color: Color) -> bool {
    let index = match color {
        Color::White => 0,
        Color::Black => 56
    };

    game.castling_rights[color][KingSide] 
    && game.pieces[color][Piece::Rook].is_set(index + 7)
    && game.empty.is_set(index + 6)
    && game.empty.is_set(index + 5)
    && !game.square_is_attacked(index + 4, !color)
    && !game.square_is_attacked(index + 5, !color)
    && !game.square_is_attacked(index + 6, !color)
}

pub fn long_castling(game: &Board, color: Color) -> bool {
    let index = match color {
        Color::White => 0,
        Color::Black => 56
    };

    game.castling_rights[color][QueenSide] 
    && game.pieces[color][Piece::Rook].is_set(index)
    && game.empty.is_set(index + 1)
    && game.empty.is_set(index + 2)
    && game.empty.is_set(index + 3)
    && !game.square_is_attacked(index + 4, !color)
    && !game.square_is_attacked(index + 3, !color)
    && !game.square_is_attacked(index + 2, !color)
}

/// Returns a bitboard of pseudolegal knight moves of a given color
pub fn knight_moves(knight: u8, game: &Board, color: Color) -> Bitboard {
    let blockers = game.friends(color);
    let mask = KNIGHT_MASKS[knight as usize];
    mask & !blockers
}

/// Returns a bitboard of pseudolegal rook moves of a given color
pub fn rook_moves(index: u8, game: &Board, color: Color) -> Bitboard {
    use DIRECTION::*;
    let e = scan_ray(index, game, color, E);
    let w = scan_ray(index, game, color, W);
    let n = scan_ray(index, game, color, N);
    let s = scan_ray(index, game, color, S);

    e | w | s | n
}

pub fn bishop_moves(index: u8, game: &Board, color: Color) -> Bitboard {
    use DIRECTION::*;
    let ne = scan_ray(index, game, color, NE);
    let nw = scan_ray(index, game, color, NW);
    let se = scan_ray(index, game, color, SE);
    let sw = scan_ray(index, game, color, SW);

    ne | nw | se | sw
}

pub fn queen_moves(queen: u8, game: &Board, color: Color) -> Bitboard {
    bishop_moves(queen, game, color) | rook_moves(queen, game, color)
}

fn scan_ray(index: u8, game: &Board, color: Color, direction: DIRECTION) -> Bitboard {
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
        DIRECTION::W | DIRECTION::S | DIRECTION::SE | DIRECTION::SW => (blockers & ray).msb_index(),
        _ => (blockers & ray).lsb_index(),
    };
    match ray_blocker {
        Some(ray_blocker) => {
            ray & !Bitboard::from(1u64) & !INCL_RAY[ray_blocker as usize][direction as usize]
        }
        None => ray,
    }
}

pub fn check_en_passant(pawn: u8, game: &Board, color: Color) -> bool {
    let enemy_pawns = game.pieces[!color][Piece::Pawn];
    let pawn = Bitboard::from(pawn);
    (enemy_pawns 
        & ((pawn << 1) & !*FILE_A)
    ) != 0 
    || (enemy_pawns 
        & ((pawn >> 1) & !*FILE_H)
    ) != 0
}
