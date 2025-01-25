use crate::{
    bitboard::{Bitboard, Piece},
    game::{Color, Game, Move},
    masks::{bishop_attacks, king_attacks, knight_attacks, pawn_attacks, rook_attacks},
};

#[test]
fn knight_moves() {
    use Color::*;
    let game = &Game::empty();

    assert_eq!(
        knight_attacks(18, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b01010000, 
            0b10001000, 
            0b00000000, 
            0b10001000,
            0b01010000,
        ])
    );

    assert_eq!(
        knight_attacks(16, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b01000000, 
            0b00100000, 
            0b00000000, 
            0b00100000,
            0b01000000,
        ])
    );

    assert_eq!(
        knight_attacks(23, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000010, 
            0b00000100, 
            0b00000000, 
            0b00000100,
            0b00000010,
        ])
    );

    assert_eq!(
        knight_attacks(7, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000010, 
            0b00000100,
            0b00000000,
        ])
    );

    assert_eq!(
        knight_attacks(56, game, White),
        Bitboard::from([
            0b00000000, 
            0b00100000, 
            0b01000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000,
            0b00000000,
        ])
    );
}

#[test]
fn pawn_tests() {
    use Color::*;
    let mut game = Game::empty();
    game.br |= Bitboard::from(1u64 << 17 | 1u64 << 19 | 1u64 << 31);
    assert_eq!(
        pawn_attacks(10, &game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00100000, 
            0b01110000, 
            0b00000000,
            0b00000000,
        ])
    );

    assert_eq!(pawn_attacks(11, &game, White), Bitboard::from(0u64));

    assert_eq!(
        pawn_attacks(12, &game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00001000, 
            0b00011000, 
            0b00000000,
            0b00000000,
        ])
    );
    assert_eq!(
        pawn_attacks(14, &game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000010, 
            0b00000010, 
            0b00000000,
            0b00000000,
        ])
    );

    assert_eq!(
        pawn_attacks(15, &game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000001, 
            0b00000000,
            0b00000000,
        ])
    );

    assert_eq!(
        pawn_attacks(16, &game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b10000000, 
            0b00000000, 
            0b00000000,
            0b00000000,
        ])
    );
}

#[test]
fn king_moves() {
    use Color::*;
    let game = &Game::empty();
    assert_eq!(
        king_attacks(7, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000011,
            0b00000010,
        ])
    );

    assert_eq!(
        king_attacks(4, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00011100,
            0b00010100,
        ])
    );

    assert_eq!(
        king_attacks(0, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b11000000,
            0b01000000,
        ])
    );

    assert_eq!(
        king_attacks(15, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000011, 
            0b00000010,
            0b00000011,
        ])
    );

    assert_eq!(
        king_attacks(8, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b11000000, 
            0b01000000,
            0b11000000,
        ])
    );

    assert_eq!(
        king_attacks(19, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00111000, 
            0b00101000, 
            0b00111000,
            0b00000000,
        ])
    );

    assert_eq!(
        king_attacks(63, game, White),
        Bitboard::from([
            0b00000010, 
            0b00000011, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000,
            0b00000000,
        ])
    );

    assert_eq!(
        king_attacks(60, game, White),
        Bitboard::from([
            0b00010100, 
            0b00011100, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000,
            0b00000000,
        ])
    );

    assert_eq!(
        king_attacks(56, game, White),
        Bitboard::from([
            0b01000000, 
            0b11000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000,
            0b00000000,
        ])
    );
}

#[test]
fn rook_moves() {
    use Color::*;
    let game = &Game::empty();
    assert_eq!(
        rook_attacks(0, game, White),
        Bitboard::from([
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000,
            0b01111111,
        ])
    );

    assert_eq!(
        rook_attacks(7, game, White),
        Bitboard::from([
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001,
            0b11111110,
        ])
    );

    assert_eq!(
        rook_attacks(56, game, White),
        Bitboard::from([
            0b01111111, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000, 
            0b10000000,
            0b10000000,
        ])
    );

    assert_eq!(
        rook_attacks(63, game, White),
        Bitboard::from([
            0b11111110, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001, 
            0b00000001,
            0b00000001,
        ])
    );

    assert_eq!(
        rook_attacks(20, game, White),
        Bitboard::from([
            0b00001000, 
            0b00001000, 
            0b00001000, 
            0b00001000, 
            0b00001000, 
            0b11110111, 
            0b00001000,
            0b00001000,
        ])
    );
}

#[test]
fn bishop_moves() {
    use Color::*;
    let game = &Game::empty();

    assert_eq!(
        bishop_attacks(20, game, White),
        Bitboard::from([
            0b00000000, 
            0b10000000, 
            0b01000001, 
            0b00100010, 
            0b00010100, 
            0b00000000, 
            0b00010100,
            0b00100010,
        ])
    );

    assert_eq!(
        bishop_attacks(5, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b10000000, 
            0b01000000, 
            0b00100000, 
            0b00010001, 
            0b00001010,
            0b00000000,
        ])
    );

    assert_eq!(
        bishop_attacks(23, game, White),
        Bitboard::from([
            0b00100000, 
            0b00010000, 
            0b00001000, 
            0b00000100, 
            0b00000010, 
            0b00000000, 
            0b00000010,
            0b00000100,
        ])
    );

    assert_eq!(
        bishop_attacks(7, game, White),
        Bitboard::from([
            0b10000000, 
            0b01000000, 
            0b00100000, 
            0b00010000, 
            0b00001000, 
            0b00000100, 
            0b00000010,
            0b00000000,
        ])
    );

    assert_eq!(
        bishop_attacks(0, game, White),
        Bitboard::from([
            0b00000001, 
            0b00000010, 
            0b00000100, 
            0b00001000, 
            0b00010000, 
            0b00100000, 
            0b01000000,
            0b00000000,
        ])
    );

    assert_eq!(
        bishop_attacks(63, game, White),
        Bitboard::from([
            0b00000000, 
            0b00000010, 
            0b00000100, 
            0b00001000, 
            0b00010000, 
            0b00100000, 
            0b01000000,
            0b10000000,
        ])
    );

    assert_eq!(
        bishop_attacks(56, game, White),
        Bitboard::from([
            0b00000000, 
            0b01000000, 
            0b00100000, 
            0b00010000, 
            0b00001000, 
            0b00000100, 
            0b00000010,
            0b00000001,
        ])
    );

    assert_eq!(
        bishop_attacks(57, game, White),
        Bitboard::from([
            0b00000000, 
            0b10100000, 
            0b00010000, 
            0b00001000, 
            0b00000100, 
            0b00000010, 
            0b00000001,
            0b00000000,
        ])
    );
}

#[test]
fn rook_blockers() {
    let mut game = Game::empty();
    game.wp |= Bitboard::from(1u64 << 28);
    game.wp |= Bitboard::from(1u64 << 23);
    game.bp |= Bitboard::from(1u64 << 4);
    game.bp |= Bitboard::from(1u64 << 18);

    assert_eq!(
        rook_attacks(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00110110, 
            0b00001000,
            0b00001000,
        ])
    );
}

#[test]
fn bishop_blockers() {
    let mut game = Game::empty();
    game.bp |= Bitboard::from(1u64 << 29);
    game.wp |= Bitboard::from(1u64 << 27);
    game.wp |= Bitboard::from(1u64 << 2);
    game.bp |= Bitboard::from(1u64 << 6);
    assert_eq!(
        bishop_attacks(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000100, 
            0b00000000, 
            0b00010100,
            0b00000010,
        ])
    );


}

#[test]
fn check() {
    use Color::*;
    let mut game = Game::empty();

    game.wk |= Bitboard::from(1u64);
    game.br |= Bitboard::from(1u64 << 8);
    assert!(game.is_check(White));

    game.br &= Bitboard::default();
    game.bb |= Bitboard::from(1u64 << 63);
    assert!(game.is_check(White));

    game.bb &= Bitboard::default();
    game.bn |= Bitboard::from(1u64 << 17);
    assert!(game.is_check(White));

    game.bn &= Bitboard::default();
    game.bp |= Bitboard::from(1u64 << 9);
    assert!(game.is_check(White));

    game.bp &= Bitboard::default();
    game.bk |= Bitboard::from(1u64 << 1);
    assert!(game.is_check(White));

    game.bk &= Bitboard::default();
    assert!(!game.is_check( White));

}

#[test]
#[should_panic]
fn binded_pieces() {
    let mut game = Game::empty();
    game.bq |= Bitboard::from(1u64 << 49);
    game.wk |= Bitboard::from(1u64 << 1);
    game.wr |= Bitboard::from(1u64 << 9);
    game.make_move(&Move::new(9, 10, Piece::Rook, Color::White, None));
}
