use crate::{
    bitboard::Bitboard,
    game::{Color, Game},
    shared::{DIRECTION, FILE_A, FILE_B, FILE_G, FILE_H, RANK_2, RANK_7, RAY},
};

pub fn pawn_advances(pawn: u32, game: &Game, color: Color) -> Bitboard {
    let pawn = 1u64 << pawn;
    let blockers = game.all_pieces().num() & !pawn;
    let advances = match color {
        Color::White => (((pawn & RANK_2) << 16) & !(blockers << 8)) | (pawn << 8),
        Color::Black => (((pawn & RANK_7) >> 16) & !(blockers >> 8)) | (pawn >> 8),
    } & !blockers;
    Bitboard::from(advances)
}

pub fn pawn_captures(pawn: u32, game: &Game, color: Color) -> Bitboard {
    let enemies = game.opposite_pieces(color).num();
    let pawn = 1u64 << pawn;
    let captures = match color {
        Color::White => ((pawn << 9) & !FILE_A) | ((pawn << 7) & !FILE_H),
        Color::Black => ((pawn >> 9) & !FILE_H) | ((pawn >> 7) & !FILE_A),
    };
    let captures = captures & enemies;
    Bitboard::from(captures)
}

pub fn pawn_attacks(pawn: u32, game: &Game, color: Color) -> Bitboard {
    pawn_advances(pawn, game, color) | pawn_captures(pawn, game, color)
}

pub fn king_attacks(king: u32, game: &Game, color: Color) -> Bitboard {
    let bnum = 1u64 << king;
    let blockers = game.own_pieces(color).num() & !bnum;
    let moves = bnum >> 8
        | bnum << 8
        | ((bnum >> 1 | bnum >> 9 | bnum << 7) & !FILE_H)
        | ((bnum >> 7 | bnum << 1 | bnum << 9) & !FILE_A);
    Bitboard::from(moves & !blockers)
}

pub fn knight_attacks(knight: u32, game: &Game, color: Color) -> Bitboard {
    let bnum = 1u64 << knight;
    let blockers = game.own_pieces(color).num() & !bnum;
    let moves = ((bnum >> 17 | bnum << 15) & !FILE_H)
        | ((bnum >> 15 | bnum << 17) & !FILE_A)
        | ((bnum >> 10 | bnum << 6) & !FILE_H & !FILE_G)
        | ((bnum >> 6 | bnum << 10) & !FILE_A & !FILE_B);
    Bitboard::from(moves & !blockers)
}

// FIXME: match clauses because bit scans could be empty, which is bad performance-wise.
// Somehow it needs to be fixed
pub fn rook_attacks(index: u32, game: &Game, color: Color) -> Bitboard {
    use DIRECTION::*;
    let friends = game.own_pieces(color).num();
    let enemies = game.opposite_pieces(color).num();

    let blockers = friends | (enemies << 1);
    let east = RAY[index as usize][E as usize];
    let east_blocker = Bitboard::lsb_index(blockers & east);
    let east_scan = match east_blocker {
        Some(east_blocker) => {
            east & !(1u64 << east_blocker) & !RAY[east_blocker as usize][E as usize]
        }
        None => east,
    };

    let blockers = friends | (enemies >> 1);
    let west = RAY[index as usize][W as usize];
    let west_blocker = Bitboard::msb_index(blockers & west);
    let west_scan = match west_blocker {
        Some(west_blocker) => {
            west & !(1u64 << west_blocker) & !RAY[west_blocker as usize][W as usize]
        }
        None => west,
    };

    let blockers = friends | (enemies << 8);
    let north = RAY[index as usize][N as usize];
    let north_blocker = Bitboard::lsb_index(blockers & north);
    let north_scan = match north_blocker {
        Some(north_blocker) => {
            north & !(1u64 << north_blocker) & !RAY[north_blocker as usize][N as usize]
        }
        None => north,
    };

    let blockers = friends | (enemies >> 8);
    let south = RAY[index as usize][S as usize];
    let south_blocker = Bitboard::msb_index(blockers & south);
    let south_scan = match south_blocker {
        Some(south_blocker) => {
            south & !(1u64 << south_blocker) & !RAY[south_blocker as usize][S as usize]
        }
        None => south,
    };

    Bitboard::from(west_scan | east_scan | north_scan | south_scan)
}

pub fn bishop_attacks(bishop: u32, game: &Game, color: Color) -> Bitboard {
    use DIRECTION::*;
    let friends = game.own_pieces(color).num();
    let enemies = game.opposite_pieces(color).num();

    let blockers = friends | enemies << 9;
    let ne = RAY[bishop as usize][NE as usize];
    let ne_blocker = Bitboard::lsb_index(blockers & ne);
    let ne_scan = match ne_blocker {
        Some(ne_blocker) => ne & !(1u64 << ne_blocker) & !RAY[ne_blocker as usize][NE as usize],
        None => ne,
    };

    let blockers = friends | enemies >> 9;
    let sw = RAY[bishop as usize][SW as usize];
    let sw_blocker = Bitboard::msb_index(blockers & sw);
    let sw_scan = match sw_blocker {
        Some(sw_blocker) => sw & !(1u64 << sw_blocker) & !RAY[sw_blocker as usize][SW as usize],
        None => sw,
    };

    let blockers = friends | enemies << 7;
    let nw = RAY[bishop as usize][NW as usize];
    let nw_blocker = Bitboard::lsb_index(blockers & nw);
    let nw_scan = match nw_blocker {
        Some(nw_blocker) => nw & !(1u64 << nw_blocker) & !RAY[nw_blocker as usize][NW as usize],
        None => nw,
    };

    let blockers = friends | enemies >> 7;
    let se = RAY[bishop as usize][SE as usize];
    let se_blocker = Bitboard::msb_index(blockers & se);
    let se_scan = match se_blocker {
        Some(se_blocker) => se & !(1u64 << se_blocker) & !RAY[se_blocker as usize][SE as usize],
        None => se,
    };

    Bitboard::from(sw_scan | ne_scan | nw_scan | se_scan)
}

pub fn queen_attacks(queen: u32, game: &Game, color: Color) -> Bitboard {
    bishop_attacks(queen, game, color) | rook_attacks(queen, game, color)
}

