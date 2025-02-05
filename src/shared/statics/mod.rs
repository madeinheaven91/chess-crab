use masks::{KING_MASKS, KNIGHT_MASKS, PAWN_CAPTURE_MASKS};
use rays::{INCL_RAY, RAY};
use zobrist::{BLACK_MOVE_KEY, CASTLING_KEYS, PIECE_KEYS};

pub mod masks;
pub mod rays;
pub mod zobrist;

pub fn init_statics() {
    let _ = RAY[0][0];
    let _ = INCL_RAY[0][0];
    let _ = PAWN_CAPTURE_MASKS[0][0];
    let _ = KING_MASKS[0];
    let _ = KNIGHT_MASKS[0];
    let _ = PIECE_KEYS[0];
    let _ = CASTLING_KEYS[0];
    let _ = BLACK_MOVE_KEY;
}
