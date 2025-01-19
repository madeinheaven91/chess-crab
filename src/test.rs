use crate::{masks::{king_attacks, knight_attacks, pawn_attacks}, bitboard::Bitboard, game::Color, shared::{BP, WP}};

#[test]
fn knight_tests() {
    let knight = Bitboard::from(1 << 21);
    assert_eq!(knight_attacks(knight), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b01010000,
        0b10001000,
        0b00000000,
        0b10001000,
        0b01010000,
    ]));

    let knight = Bitboard::from(1 << 23);
    assert_eq!(knight_attacks(knight), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b01000000,
        0b00100000,
        0b00000000,
        0b00100000,
        0b01000000,
    ]));

    let knight = Bitboard::from(1 << 16);
    assert_eq!(knight_attacks(knight), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000010,
        0b00000100,
        0b00000000,
        0b00000100,
        0b00000010,
    ]));

    
    let knight = Bitboard::from(1);
    assert_eq!(knight_attacks(knight), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000010,
        0b00000100,
        0b00000000,
    ]));

    let knight = Bitboard::from(1 << 63);
    assert_eq!(knight_attacks(knight), Bitboard::from([
        0b00000000,
        0b00100000,
        0b01000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ]));
}

#[test]
fn pawn_tests(){
    assert_eq!(pawn_attacks(Bitboard::from(BP), Color::Black), Bitboard::from([
        0b00000000,
        0b00000000,
        0b11111111,
        0b11111111,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ]));
    assert_eq!(pawn_attacks(Bitboard::from(WP), Color::White), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b11111111,
        0b11111111,
        0b00000000,
        0b00000000,
    ]));

    let random_wpawns = Bitboard::from([
        0b00000000,
        0b10000000,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00001000,
        0b00000110,
        0b00000000,
    ]);
    assert_eq!(pawn_attacks(random_wpawns, Color::White), Bitboard::from([
        0b10000000,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00001110,
        0b00000110,
        0b00000000,
        0b00000000,
    ]));

    let random_bpawns = Bitboard::from([
        0b00000000,
        0b00000110,
        0b10000000,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00001000,
        0b00000000,
    ]);
    assert_eq!(pawn_attacks(random_bpawns, Color::Black), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000110,
        0b10000110,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00001000,
    ]));
}

#[test]
fn king_tests(){
    let king = Bitboard::from(1);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000011,
        0b00000010,
    ]));
    let king = Bitboard::from(1 << 3);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00011100,
        0b00010100,
    ]));
    let king = Bitboard::from(1 << 7);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b11000000,
        0b01000000,
    ]));
    let king = Bitboard::from(1 << 8);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000011,
        0b00000010,
        0b00000011,
    ]));
    let king = Bitboard::from(1 << 15);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b11000000,
        0b01000000,
        0b11000000,
    ]));
    let king = Bitboard::from(1 << 20);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00111000,
        0b00101000,
        0b00111000,
        0b00000000,
    ]));
    let king = Bitboard::from(1 << 56);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00000010,
        0b00000011,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ]));
    let king = Bitboard::from(1 << 59);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b00010100,
        0b00011100,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ]));
    let king = Bitboard::from(1 << 63);
    assert_eq!(king_attacks(king), Bitboard::from([
        0b01000000,
        0b11000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ]));
}
