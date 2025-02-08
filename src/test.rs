use crate::game::{moves::{individual::{all_pawn_captures, bishop_moves, king_moves, knight_moves, pawn_moves, rook_moves}, move_struct::{Flag, Move}}, structs::{bitboard::Bitboard, board::Board, color::Color, piece::Piece}};


#[test]
fn knight() {
    use Color::*;
    let game = &Board::empty();

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
    let mut game = Board::empty();
    game.pieces[Black][Pawn].set_1(17);
    game.pieces[Black][Pawn].set_1(19);
    game.pieces[Black][Pawn].set_1(31);
    game.update_pieces();
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
    let game = &Board::empty();
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
    let game = &Board::empty();
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
    let game = &Board::empty();

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
    let mut game = Board::empty();
    game.pieces[White][Pawn].set_1(36);
    game.pieces[White][Pawn].set_1(22);
    game.pieces[White][Pawn].set_1(4);
    game.pieces[White][Pawn].set_1(18);
    game.update_pieces();


    assert_eq!(
        rook_moves(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00001000, 
            0b00010100, 
            0b00001000,
            0b00000000,
        ])
    );
    game.pieces[White][Pawn].set_1(17);
    game.pieces[White][Pawn].set_1(23);
    game.pieces[White][Pawn].set_1(44);
    game.update_pieces();
    assert_eq!(
        rook_moves(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00001000, 
            0b00010100, 
            0b00001000,
            0b00000000,
        ])
    );

    let mut game = Board::empty();
    game.pieces[Black][Pawn].set_1(28);
    game.pieces[Black][Pawn].set_1(19);
    game.pieces[Black][Pawn].set_1(12);
    game.pieces[Black][Pawn].set_1(21);
    game.update_pieces();


    assert_eq!(
        rook_moves(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00001000, 
            0b00010100, 
            0b00001000,
            0b00000000,
        ])
    );
}

#[test]
fn bishop_blockers() {
    use Color::*;
    use Piece::*;
    let mut game = Board::empty();
    game.pieces[White][Pawn].set_1(38);
    game.pieces[White][Pawn].set_1(34);
    game.pieces[White][Pawn].set_1(2);
    game.pieces[White][Pawn].set_1(6);
    game.update_pieces();
    assert_eq!(
        bishop_moves(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00010100, 
            0b00000000, 
            0b00010100,
            0b00000000,
        ])
    );

    let mut game = Board::empty();
    game.pieces[Black][Pawn].set_1(38);
    game.pieces[Black][Pawn].set_1(34);
    game.pieces[Black][Pawn].set_1(2);
    game.pieces[Black][Pawn].set_1(6);
    game.update_pieces();
    assert_eq!(
        bishop_moves(20, &game, Color::White),
        Bitboard::from([
            0b00000000, 
            0b00000000, 
            0b00000000, 
            0b00100010, 
            0b00010100, 
            0b00000000, 
            0b00010100,
            0b00100010,
        ])
    );
}

#[test]
fn check() {
    use Color::*;
    use Piece::*;
    let mut game = Board::empty();

    game.pieces[White][King].set_1(0);
    game.pieces[Black][King].set_1(60);
    game.pieces[Black][Rook].set_1(8);
    assert_eq!(game.is_check(), Some(White));

    game.pieces[Black][Rook].clear();
    game.pieces[Black][Bishop].set_1(63);
    assert_eq!(game.is_check(), Some(White));

    game.pieces[Black][Bishop].clear();
    game.pieces[Black][Knight].set_1(17);
    assert_eq!(game.is_check(), Some(White));

    game.pieces[Black][Knight].clear();
    game.pieces[Black][Pawn].set_1(9);
    // assert!(game.is_check().is_none());
    assert_eq!(all_pawn_captures(game.pieces[Black][Pawn], Black).num(), 5);
    assert_eq!(game.is_check(), Some(White));

    game.pieces[Black][Pawn].clear();
    game.pieces[Black][King].clear();
    game.pieces[Black][King].set_1(1);
    assert!(game.is_check() == Some(White));

    game.pieces[Black][King].clear();
    game.pieces[Black][King].set_1(60);
    assert_eq!(game.is_check(), None);

}

#[test]
#[should_panic]
fn binded_pieces() {
    use Color::*;
    use Piece::*;
    let mut game = Board::empty();
    game.pieces[Black][Queen].set_1(49);
    game.pieces[White][King].set_1(1);
    game.pieces[White][Rook].set_1(9);
    let res = game.make_move(&Move::new(&game, 9, 10, Piece::Rook, Color::White));
    if res.is_ok() { panic!() }
}

#[test]
fn promotion(){
    use Color::*;
    use Piece::*;

    let mut game = Board::empty();

    game.pieces[White][King].set_1(1);
    game.pieces[Black][King].set_1(10);
    game.pieces[White][Pawn].set_1(56);
    let _ = game.make_move(&Move::promotion(&game, 56, 63, White, Queen));

    assert_eq!(game.pieces[White][Queen].num(), (1u64 << 63));
    assert_eq!(game.pieces[White][Pawn].num(), 0);
}

#[test]
fn capture_promotion(){
    use Color::*;
    use Piece::*;

    let mut game = Board::empty();
    game.pieces[White][Pawn].set_1(56);
    game.pieces[Black][Knight].set_1(62);
    game.pieces[White][King].set_1(1);
    game.pieces[Black][King].set_1(10);
    let _ = game.make_move(&Move::promotion(&game, 56, 62, White, Queen));

    assert_eq!(game.pieces[White][Queen].num(), (1u64 << 62));
    assert_eq!(game.pieces[White][Pawn].num(), 0);
    assert_eq!(game.pieces[Black][Knight].num(), 0);
}

#[test]
fn move_flag_detection() {
    use Color::*;
    use Piece::*;

    let mut game = Board::empty();
    game.pieces[White][King].set_1(0);
    game.pieces[Black][King].set_1(63);

    let mv = Move::new(&game, 0, 1, King, White);
    assert_eq!(mv.flag, Flag::Default);

    game.pieces[Black][Pawn].set_1(1);
    let mv = Move::new(&game, 0, 1, King, White);
    assert_eq!(mv.flag, Flag::Capture(Pawn));

    let mut game = Board::empty();
    game.pieces[White][Rook].set_1(0);
    game.pieces[Black][King].set_1(63);

    let mv = Move::short_castling(4, White);
    assert_eq!(mv.flag, Flag::ShortCastling);
    let mv = Move::long_castling(4, White);
    assert_eq!(mv.flag, Flag::LongCastling);

    game.pieces[White][Pawn].set_1(8);
    let mv = Move::new(&game, 8, 24, Pawn, White);
    assert_eq!(mv.flag, Flag::LongPawnMove);

    game.pieces[Black][Pawn].set_1(17);
    let mv = Move::new(&game, 8, 17, Pawn, White);
    assert_eq!(mv.flag, Flag::Capture(Pawn));

    game.pieces[White][Pawn].set_1(48);
    game.pieces[Black][Pawn].set_1(57);
    let mv = Move::promotion(&game, 48, 57, White, Knight);
    assert_eq!(mv.flag, Flag::CapturePromotion(Pawn, Knight));
    let mv = Move::promotion(&game, 48, 56, White, Knight);
    assert_eq!(mv.flag, Flag::Promotion(Knight));

    let mv = Move::null();
    assert_eq!(mv.flag, Flag::Null);
}

// [nodes, captures, en_passant, castles, promotions, checks, checkmates(?)]
fn perft_rec(game: &Board, depth: u32, current_depth: u32) -> [usize; 6] {
    let mut perft_res = [0; 6];
    let legal_moves = game.gen_legal_moves();
    if legal_moves.is_empty() {
        return [0; 6]
    }
    if current_depth == depth {
        for mv in legal_moves{
            perft_res[0] += 1;
            let mut new_game = game.clone();
            let _ = new_game.make_move(&mv);
            match mv.flag {
                // Flag::Default | Flag::LongPawnMove => perft_res[0] += 1,
                Flag::Capture(_) => {
                    perft_res[1] += 1;
                    // println!("depth: {}, current_depth: {}, move: {:?}", depth, current_depth, mv);
                }
                Flag::EnPassant => { 
                    perft_res[1] += 1; perft_res[2] += 1;
                    // println!("depth: {}, current_depth: {}, move: {:?}", depth, current_depth, mv);
                    // println!("prev: {}", game);
                    // println!("next: {}", new_game);
                },
                Flag::Promotion(_) => {
                    perft_res[4] += 1;
                    // println!("depth: {}, current_depth: {}, move: {:?}", depth, current_depth, mv);
                    // println!("{}", new_game);
                },
                Flag::CapturePromotion(_, _) => { 
                    perft_res[4] += 1; perft_res[1] += 1;
                    // println!("depth: {}, current_depth: {}, move: {:?}", depth, current_depth, mv);
                    // println!("{}", new_game);
                },
                Flag::LongCastling | Flag::ShortCastling => {
                    perft_res[3] += 1;
                    // println!("depth: {}, current_depth: {}, move: {:?}", depth, current_depth, mv);
                    // println!("{}", new_game);
                }
                _ => ()
            }
            if new_game.is_check().is_some() { 
                perft_res[5] += 1;
            };
        };

    }else{
        for mv in legal_moves{
            let mut new_game = game.clone();
            let _ = new_game.make_move(&mv);
            for (perft_res, new) in perft_res.iter_mut().zip(perft_rec(&new_game, depth, current_depth + 1)){
                *perft_res += new;
            }
        }
    }
    perft_res
}

/// PERFT TEST RESULTS ARE TAKEN FROM
/// https://www.chessprogramming.org/Perft_Results
#[test]
fn perft_init(){
    let game = Board::default();

    let perft = perft_rec(&game, 1, 1);
    assert_eq!(perft, [20, 0,  0, 0, 0, 0]);

    let perft = perft_rec(&game, 2, 1);
    assert_eq!(perft, [400, 0, 0, 0, 0, 0]);

    let perft = perft_rec(&game, 3, 1);
    assert_eq!(perft, [8902, 34, 0, 0, 0, 12]);

    let perft = perft_rec(&game, 4, 1);
    assert_eq!(perft, [197281, 1576, 0, 0, 0, 469]);

    // not fast enough for this
    // let nodes = perft_rec(&game, 5, 1);
    // assert_eq!(nodes, 4865609);
}

#[test]
fn kiwipete(){
    let game = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

    let perft = perft_rec(&game, 1, 1);
    assert_eq!(perft, [48, 8, 0, 2, 0, 0]);
    let perft = perft_rec(&game, 2, 1);
    assert_eq!(perft, [2039, 351, 1, 91, 0, 3]);
    let perft = perft_rec(&game, 3, 1);
    assert_eq!(perft, [97862, 17102, 45, 3162, 0, 993]);
    let perft = perft_rec(&game, 4, 1);
    assert_eq!(perft, [4085603, 757163, 1929, 128013, 15172, 25523]);
}

#[test]
fn perft_pos_3(){
    let game = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

    let perft = perft_rec(&game, 1, 1);
    assert_eq!(perft, [14, 1, 0, 0, 0, 2]);
    let perft = perft_rec(&game, 2, 1);
    assert_eq!(perft, [191, 14, 0, 0, 0, 10]);
    let perft = perft_rec(&game, 3, 1);
    assert_eq!(perft, [2812, 209, 2, 0, 0, 267]);
    let perft = perft_rec(&game, 4, 1);
    assert_eq!(perft, [43238, 3348, 123, 0, 0, 1680]);
    let perft = perft_rec(&game, 5, 1);
    assert_eq!(perft, [674624, 52051, 1165, 0, 0, 52950]);
}

#[test]
fn perft_pos_4(){
    let game = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();

    let perft = perft_rec(&game, 1, 1);
    assert_eq!(perft, [6, 0, 0, 0, 0, 0]);
    let perft = perft_rec(&game, 2, 1);
    assert_eq!(perft, [264, 87, 0, 6, 48, 10]);
    let perft = perft_rec(&game, 3, 1);
    assert_eq!(perft, [9467, 1021, 4, 0, 120, 38]);
    let perft = perft_rec(&game, 4, 1);
    assert_eq!(perft[0], 422333);
}

#[test]
fn perft_pos_5(){
    let game = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let perft = perft_rec(&game, 1, 1);
    assert_eq!(perft[0], 44);
    let perft = perft_rec(&game, 2, 1);
    assert_eq!(perft[0], 1486);
    let perft = perft_rec(&game, 3, 1);
    assert_eq!(perft[0], 62379);
}

#[test]
fn perft_pos_6(){
    let game = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
    let perft = perft_rec(&game, 1, 1)[0];
    assert_eq!(perft, 46);
    let perft = perft_rec(&game, 2, 1)[0];
    assert_eq!(perft, 2079);
    let perft = perft_rec(&game, 3, 1)[0];
    assert_eq!(perft, 89890);
    let perft = perft_rec(&game, 4, 1)[0];
    assert_eq!(perft,  3894594);
}
