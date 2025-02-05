use lazy_static::lazy_static;

// ZOBRIST HASH
pub static ZOBRIST_SEED: u64 = 0xf7328ab2;
pub static ZOBRIST_MULTIPLIER: u64 = 0x923bc23a0f;
pub static ZOBRIST_SUMMAND: u64 = 0x9940fc28ae2b;
lazy_static! {
    pub static ref PIECE_KEYS: [[[u64; 6]; 2]; 64] = Zobrist::constants();
    pub static ref BLACK_MOVE_KEY: u64 = Zobrist::next(PIECE_KEYS[63][1][5]);
    pub static ref CASTLING_KEYS: [[u64; 2]; 2] = Zobrist::castlings(*BLACK_MOVE_KEY);
}

struct Zobrist {}

impl Zobrist {

    fn next(prev: u64) -> u64 {
        prev.wrapping_mul(ZOBRIST_MULTIPLIER).wrapping_add(ZOBRIST_SUMMAND)
    }

    fn constants() -> [[[u64; 6]; 2]; 64] {
        let mut res = [[[0; 6]; 2]; 64];
        let mut prev = ZOBRIST_SEED;
        (0..64).for_each(|square| {
            (0..2).for_each(|side| {
                (0..6).for_each(|piece| {
                    prev = Zobrist::next(prev);
                    res[square][side][piece] = Zobrist::next(prev);
                });
            });
        });
        res
    }

    fn castlings(prev: u64) -> [[u64; 2]; 2]{
        let white_kingside = Zobrist::next(prev);
        let white_queenside = Zobrist::next(white_kingside);
        let black_kingside = Zobrist::next(white_queenside);
        let black_queenside = Zobrist::next(black_kingside);
        [
            [white_kingside, white_queenside],
            [black_kingside, black_queenside]
        ]
    }
}
