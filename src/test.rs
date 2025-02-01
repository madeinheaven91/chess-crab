use crate::game::{
    moves::{bishop_moves, king_moves, knight_moves, pawn_moves, rook_moves}, structs::{bitboard::Bitboard, color::Color, game::Game, movement::Move, piece::Piece}
};

#[test]
fn knight() {
    use Color::*;
    let game = &Game::empty();

    assert_eq!(
        knight_moves(18, game, White),
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
        knight_moves(16, game, White),
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
        knight_moves(23, game, White),
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
        knight_moves(7, game, White),
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
        knight_moves(56, game, White),
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
    use Piece::*;
    let mut game = Game::empty();
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 17 | 1u64 << 19 | 1u64 << 31);
    assert_eq!(
        pawn_moves(10, &game, White),
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

    assert_eq!(pawn_moves(11, &game, White), Bitboard::from(0u64));

    assert_eq!(
        pawn_moves(12, &game, White),
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
        pawn_moves(14, &game, White),
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
        pawn_moves(15, &game, White),
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
        pawn_moves(16, &game, White),
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
fn king() {
    use Color::*;
    let game = &Game::empty();
    assert_eq!(
        king_moves(7, game, White),
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
        king_moves(4, game, White),
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
        king_moves(0, game, White),
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
        king_moves(15, game, White),
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
        king_moves(8, game, White),
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
        king_moves(19, game, White),
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
        king_moves(63, game, White),
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
        king_moves(60, game, White),
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
        king_moves(56, game, White),
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
fn rook() {
    use Color::*;
    let game = &Game::empty();
    assert_eq!(
        rook_moves(0, game, White),
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
        rook_moves(7, game, White),
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
        rook_moves(56, game, White),
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
        rook_moves(63, game, White),
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
        rook_moves(20, game, White),
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
fn bishop() {
    use Color::*;
    let game = &Game::empty();

    assert_eq!(
        bishop_moves(20, game, White),
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
        bishop_moves(5, game, White),
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
        bishop_moves(23, game, White),
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
        bishop_moves(7, game, White),
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
        bishop_moves(0, game, White),
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
        bishop_moves(63, game, White),
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
        bishop_moves(56, game, White),
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
        bishop_moves(57, game, White),
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
    use Color::*;
    use Piece::*;
    let mut game = Game::empty();
    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 28);
    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 23);
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 4);
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 18);

    assert_eq!(
        rook_moves(20, &game, Color::White),
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
    use Color::*;
    use Piece::*;
    let mut game = Game::empty();
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 29);
    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 27);
    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 2);
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 6);
    assert_eq!(
        bishop_moves(20, &game, Color::White),
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
    use Piece::*;
    let mut game = Game::empty();

    game.pieces[White][King] |= Bitboard::from(1u64);
    game.pieces[Black][Rook] |= Bitboard::from(1u64 << 8);
    assert!(game.is_check(White));

    game.pieces[Black][Rook] &= Bitboard::default();
    game.pieces[Black][Bishop] |= Bitboard::from(1u64 << 63);
    assert!(game.is_check(White));

    game.pieces[Black][Bishop] &= Bitboard::default();
    game.pieces[Black][Knight] |= Bitboard::from(1u64 << 17);
    assert!(game.is_check(White));

    game.pieces[Black][Knight] &= Bitboard::default();
    game.pieces[Black][Pawn] |= Bitboard::from(1u64 << 9);
    assert!(game.is_check(White));

    game.pieces[Black][Pawn] &= Bitboard::default();
    game.pieces[Black][King] |= Bitboard::from(1u64 << 1);
    assert!(game.is_check(White));

    game.pieces[Black][King] &= Bitboard::default();
    assert!(!game.is_check( White));

}

#[test]
#[should_panic]
fn binded_pieces() {
    use Color::*;
    use Piece::*;
    let mut game = Game::empty();
    game.pieces[Black][Queen] |= Bitboard::from(1u64 << 49);
    game.pieces[White][King] |= Bitboard::from(1u64 << 1);
    game.pieces[White][Rook] |= Bitboard::from(1u64 << 9);
    game.make_move(&Move::new(&game, 9, 10, Piece::Rook, Color::White));
}

#[test]
fn promotion(){
    use Color::*;
    use Piece::*;

    let mut game = Game::empty();

    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 56);
    game.make_move(&Move::promotion(&game, 56, 63, Pawn, White, Queen));

    assert_eq!(game.pieces[White][Queen].num(), (1u64 << 63));
    assert_eq!(game.pieces[White][Pawn].num(), 0);
}

#[test]
fn capture_promotion(){
    use Color::*;
    use Piece::*;

    let mut game = Game::empty();
    game.pieces[White][Pawn] |= Bitboard::from(1u64 << 56);
    game.pieces[Black][Knight] |= Bitboard::from(1u64 << 62);
    game.make_move(&Move::promotion(&game, 56, 62, Pawn, White, Queen));

    assert_eq!(game.pieces[White][Queen].num(), (1u64 << 62));
    assert_eq!(game.pieces[White][Pawn].num(), 0);
    assert_eq!(game.pieces[Black][Knight].num(), 0);
}
