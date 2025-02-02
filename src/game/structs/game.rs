use super::{bitboard::Bitboard, color::Color, game_state::GameState, movement::Move, piece::Piece};

use crate::{game::{moves::{all_pawn_captures, check_en_passant, long_castling, short_castling}, structs::{color::Castling, movement::Flag}}, shared::{consts::{BB, BK, BLACK_MOVE_KEY, BN, BP, BQ, BR, CASTLING_KEYS, PIECE_KEYS, WB, WK, WN, WP, WQ, WR}, errors::ChessError, functions::{index_to_square, lsb_index, square_to_index}}};

use crate::game::moves::{
        bishop_moves, king_moves, knight_moves, pawn_moves, queen_moves, rook_moves
    };

use std::{cell::RefCell, fmt::Display, ops::AddAssign, rc::Rc};
use Color::*;
use Piece::*;
use Castling::*;

#[derive(Clone)]
pub struct Game {
    pub pieces: [[Bitboard; 6]; 2],
    pub turn: Color,
    pub check: Option<Color>,
    pub castling_rights:[[bool; 2]; 2],
    pub en_passant: Option<u32>,
    pub move_history: Vec<Move>,
    pub halfmove_clock: u8,
    pub state: GameState,
    pub repetition_history: Vec<u64>
}

impl Game {
    pub fn empty() -> Game {
        Game {
            pieces: [[Bitboard::empty(); 6]; 2],
            turn: White,
            check: None,
            castling_rights: [[false; 2]; 2],
            en_passant: None,
            move_history: Vec::new(),
            halfmove_clock: 0,
            state: GameState::Ongoing,
            repetition_history: Vec::new()
        }
    }

    pub fn black_pieces(&self) -> Bitboard {
        self.pieces[Black][King]
        | self.pieces[Black][Queen]
        | self.pieces[Black][Rook]
        | self.pieces[Black][Bishop]
        | self.pieces[Black][Knight]
        | self.pieces[Black][Pawn]
    }
    pub fn white_pieces(&self) -> Bitboard {
        self.pieces[White][King]
        | self.pieces[White][Queen]
        | self.pieces[White][Rook]
        | self.pieces[White][Bishop]
        | self.pieces[White][Knight]
        | self.pieces[White][Pawn]
    }
    pub fn all_pieces(&self) -> Bitboard {
        self.white_pieces() | self.black_pieces()
    }
    pub fn enemies(&self, color: Color) -> Bitboard {
        match color {
            White => self.black_pieces(),
            Black => self.white_pieces(),
        }
    }

    pub fn friends(&self, color: Color) -> Bitboard {
        match color {
            White => self.white_pieces(),
            Black => self.black_pieces(),
        }
    }

    pub fn from_fen(fen: &str) -> Result<Game, ChessError> {
        let mut res = Game::empty();
        let elements = fen.split(" ").collect::<Vec<&str>>();
        // 1 - board
        // 2 - turn
        // 3 - castling
        // 4 - en passant
        // 5 - halfmove clock
        // 6 - N of full moves

        // Parse board
        for (i, char) in elements[0]
            .split("/")
            .map(|row| row.chars().rev().collect::<String>()) 
            .collect::<Vec<String>>()
            .join("")
            .replace("8", "........")
            .replace("7", ".......")
            .replace("6", "......")
            .replace("5", ".....")
            .replace("4", "....")
            .replace("3", "...")
            .replace("2", "..")
            .replace("1", ".")
            .chars()
            .rev()
            .enumerate()
        {
            if char.is_ascii_alphabetic() {
                match char {
                    'P' => res.pieces[White][Pawn] |= Bitboard::from(1u64 << i),
                    'N' => res.pieces[White][Knight] |= Bitboard::from(1u64 << i),
                    'B' => res.pieces[White][Bishop] |= Bitboard::from(1u64 << i),
                    'R' => res.pieces[White][Rook] |= Bitboard::from(1u64 << i),
                    'Q' => res.pieces[White][Queen] |= Bitboard::from(1u64 << i),
                    'K' => res.pieces[White][King] |= Bitboard::from(1u64 << i),
                    'p' => res.pieces[Black][Pawn] |= Bitboard::from(1u64 << i),
                    'n' => res.pieces[Black][Knight] |= Bitboard::from(1u64 << i),
                    'b' => res.pieces[Black][Bishop] |= Bitboard::from(1u64 << i),
                    'r' => res.pieces[Black][Rook] |= Bitboard::from(1u64 << i),
                    'q' => res.pieces[Black][Queen] |= Bitboard::from(1u64 << i),
                    'k' => res.pieces[Black][King] |= Bitboard::from(1u64 << i),
                    _ => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid piece character: {char}")))
                }
            }else{
                continue
            }
        }

        // Parse turn
        match elements[1] {
            "w" => res.turn = White,
            "b" => res.turn = Black,
            _ => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid turn character: {}", elements[1])))
        }

        // Parse castling rights
        for char in elements[2].chars(){
            match char {
                'K' => res.castling_rights[White][KingSide] = true,
                'Q' => res.castling_rights[White][QueenSide] = true,
                'k' => res.castling_rights[Black][KingSide] = true,
                'q' => res.castling_rights[Black][QueenSide] = true,
                '-' => continue,
            _ => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid castling right character: {char}")))
            }
        }

        // Parse en passant
        let en_passant = match elements[3] {
            "-" => None,
            _ => Some(square_to_index(elements[3])?)
        };
        res.en_passant = en_passant;

        // Parse halfmove clock
        let halfmove_clock = match elements[4].parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid halfmove clock: {}", elements[4])))
        };
        res.halfmove_clock = halfmove_clock;

        // Parse move count
        // NOTE: idk what to do with it
        let _move_count = match elements[5].parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid move count: {}", elements[5]))),
        };

        res.state = res.check_state();
        Ok(res)
    }

    pub fn to_fen(&self) -> String {
        // 1 - board
        // 2 - turn
        // 3 - castling
        // 4 - en passant
        // 5 - halfmove clock
        // 6 - N of full moves

        let mut res: Vec<String> = vec![String::new(); 6];
        let empty_counter = Rc::new(RefCell::new(0));

        let mut board_string = String::new();
        for i in (0..8).rev(){
            for k in 0..8{
                if let Some(p) = self.find_piece(White, i * 8 + k){
                    if !empty_counter.borrow().eq(&0){
                        board_string.push_str(&empty_counter.borrow().to_string());
                        empty_counter.replace_with(|_| 0);
                    }
                    board_string.push(p.char());
                }else if let Some(p) = self.find_piece(Black, i * 8 + k){
                    if !empty_counter.borrow().eq(&0){
                        board_string.push_str(&empty_counter.borrow().to_string());
                        empty_counter.replace_with(|_| 0);
                    }
                    board_string.push(p.char().to_ascii_lowercase());
                }else{
                    empty_counter.borrow_mut().add_assign(1);
                }
            }
            if !empty_counter.borrow().eq(&0){
                board_string.push_str(&empty_counter.borrow().to_string());
            }
            board_string.push('/');
            empty_counter.replace_with(|_| 0);
        };
        board_string.pop();
        res[0] = board_string;

        res[1] = match self.turn{
            White => "w".to_string(),
            Black => "b".to_string()
        };

        res[2] = match self.castling_rights[White][KingSide]
            || self.castling_rights[White][QueenSide]
            || self.castling_rights[Black][KingSide]
            || self.castling_rights[Black][QueenSide] {
            false => "-".to_string(),
            true => format!("{}{}{}{}",
                if self.castling_rights[White][KingSide] { "K" } else { "" },
                if self.castling_rights[White][QueenSide] { "Q" } else { "" },
                if self.castling_rights[Black][KingSide] { "k" } else { "" },
                if self.castling_rights[Black][QueenSide] { "q" } else { "" }
            )
        };

        res[3] = match self.en_passant {
            Some(val) => index_to_square(val).to_string(),
            None => "-".to_string()
        };

        res[4] = self.halfmove_clock.to_string();
        res[5] = (self.move_history.len() / 2 + 1).to_string();

        res.join(" ")

    }
    
    pub fn find_piece(&self, color: Color, index: u32) -> Option<Piece>{
        for (i, piece) in self.pieces[color].iter().enumerate(){
            if piece.num() & (1u64 << index) != 0 {
                return Some(Piece::from(i))
            }
        };
        None
    }

    /// Generates a list of all possible moves in a current position
    pub fn gen_moves(&self) -> Vec<Move> {
        let mut res = vec![];
        match self.turn {
            White => {
                for p in self.pieces[White][King] {
                    for m in king_moves(p, self, White) {
                        let mv = Move::new(self, p, m, King, White);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                    if short_castling(p, self, White) != Bitboard::empty() {
                        let mv = Move::short_castling(p, White);
                        res.push(mv)
                    }
                    if long_castling(p, self, White) != Bitboard::empty() {
                        let mv = Move::long_castling(p, White);
                        res.push(mv)
                    }
                }
                for p in self.pieces[White][Queen] {
                    for m in queen_moves(p, self, White) {
                        let mv = Move::new(self, p, m, Queen, White);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[White][Rook] {
                    for m in rook_moves(p, self, White) {
                        let mv = Move::new(self, p, m, Rook, White);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[White][Bishop] {
                    for m in bishop_moves(p, self, White) {
                        let mv = Move::new(self, p, m, Bishop, White);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[White][Knight] {
                    for m in knight_moves(p, self, White) {
                        let mv = Move::new(self, p, m, Knight, White);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[White][Pawn] {
                    for m in pawn_moves(p, self, White) {
                        if m / 8 == 7 {
                            // TODO: promotion
                            for piece in Piece::promotable() {
                                let mv = Move::promotion(self, p, m, Pawn, White, piece);
                                if self.is_legal(&mv) {
                                    res.push(mv);
                                }
                            }
                        } else {
                                let mv = Move::new(self, p, m, Pawn, White);
                            if self.is_legal(&mv) {
                                res.push(mv);
                            }
                        }
                    }
                }
            }
            Black => {
                for p in self.pieces[Black][King] {
                    for m in king_moves(p, self, Black) {
                        let mv = Move::new(self, p, m, King, Black);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                    if short_castling(p, self, Black) != Bitboard::empty() {
                        let mv = Move::short_castling(p, Black);
                        res.push(mv)
                    }
                    if long_castling(p, self, White) != Bitboard::empty() {
                        let mv = Move::long_castling(p, White);
                        res.push(mv)
                    }
                }
                for p in self.pieces[Black][Queen] {
                    for m in queen_moves(p, self, Black) {
                        let mv = Move::new(self, p, m, Queen, Black);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[Black][Rook] {
                    for m in rook_moves(p, self, Black) {
                        let mv = Move::new(self, p, m, Rook, Black);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[Black][Bishop] {
                    for m in bishop_moves(p, self, Black) {
                        let mv = Move::new(self, p, m, Bishop, Black);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[Black][Knight] {
                    for m in knight_moves(p, self, Black) {
                        let mv = Move::new(self, p, m, Knight, Black);
                        if self.is_legal(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.pieces[Black][Pawn] {
                    for m in pawn_moves(p, self, Black) {
                        if m / 8 == 0 {
                            for piece in Piece::promotable() {
                                // TODO: promotion
                                let mv = Move::promotion(self, p, m, Pawn, Black, piece);
                                if self.is_legal(&mv) {
                                    res.push(mv);
                                }
                            }
                        } else {
                            let mv = Move::new(self, p, m, Pawn, Black);
                            if self.is_legal(&mv) {
                                res.push(mv);
                            }
                        }
                    }
                }
            }
        }
        res
    }

    /// Parses an algebraically notated move into `Move`.
    /// Returns Err if the move string is invalid
    pub fn parse_move(&self, mv: &str) -> Result<Move, ChessError> {
        match mv.len() {
            4 => {
                let (start, end) = mv.split_at(2);
                let start = square_to_index(start).unwrap();
                let end = square_to_index(end).unwrap();

                // Find piece
                let (piece, color) = {
                    if self.pieces[White][King].get_bit(start) {
                        (King, White)
                    } else if self.pieces[White][Queen].get_bit(start) {
                        (Queen, White)
                    } else if self.pieces[White][Rook].get_bit(start) {
                        (Rook, White)
                    } else if self.pieces[White][Bishop].get_bit(start) {
                        (Bishop, White)
                    } else if self.pieces[White][Knight].get_bit(start) {
                        (Knight, White)
                    } else if self.pieces[White][Pawn].get_bit(start) {
                        (Pawn, White)
                    } else if self.pieces[Black][King].get_bit(start) {
                        (King, Black)
                    } else if self.pieces[Black][Queen].get_bit(start) {
                        (Queen, Black)
                    } else if self.pieces[Black][Rook].get_bit(start) {
                        (Rook, Black)
                    } else if self.pieces[Black][Bishop].get_bit(start) {
                        (Bishop, Black)
                    } else if self.pieces[Black][Knight].get_bit(start) {
                        (Knight, Black)
                    } else if self.pieces[Black][Pawn].get_bit(start) {
                        (Pawn, Black)
                    } else {
                        return Err(ChessError::InvalidMove(format!(
                            "No pieces can make move: {:?}",
                            mv
                        )));
                    }
                };

                Ok(Move::new(self, start, end, piece, color))
            }
            5 => {
                let chars = mv.chars().collect::<Vec<char>>();
                let start = square_to_index(&chars[0..2].iter().collect::<String>()).unwrap();
                let end = square_to_index(&chars[2..4].iter().collect::<String>()).unwrap();
                let promotion = match chars[4] {
                    'q' => Queen,
                    'r' => Rook,
                    'b' => Bishop,
                    'n' => Knight,
                    _ => return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv))),
                };

                // Find piece
                let (piece, color) = {
                    if self.pieces[White][King].get_bit(start) {
                        (King, White)
                    } else if self.pieces[White][Queen].get_bit(start) {
                        (Queen, White)
                    } else if self.pieces[White][Rook].get_bit(start) {
                        (Rook, White)
                    } else if self.pieces[White][Bishop].get_bit(start) {
                        (Bishop, White)
                    } else if self.pieces[White][Knight].get_bit(start) {
                        (Knight, White)
                    } else if self.pieces[White][Pawn].get_bit(start) {
                        (Pawn, White)
                    } else if self.pieces[Black][King].get_bit(start) {
                        (King, Black)
                    } else if self.pieces[Black][Queen].get_bit(start) {
                        (Queen, Black)
                    } else if self.pieces[Black][Rook].get_bit(start) {
                        (Rook, Black)
                    } else if self.pieces[Black][Bishop].get_bit(start) {
                        (Bishop, Black)
                    } else if self.pieces[Black][Knight].get_bit(start) {
                        (Knight, Black)
                    } else if self.pieces[Black][Pawn].get_bit(start) {
                        (Pawn, Black)
                    } else {
                        return Err(ChessError::InvalidMove(format!(
                            "No pieces can make move: {}",
                            mv
                        )));
                    }
                };

                if piece != Pawn {
                    return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv)));
                }

                // Ok(Move {
                //     start,
                //     end,
                //     piece: Pawn,
                //     color,
                //     promotion,
                // })

                Ok(Move::promotion(self, start, end, piece, color, promotion))
            }
            _ => Err(ChessError::InvalidMove(format!("Invalid move: {}", mv))),
        }
    }

    /// Simulates a halfmove and returns whether it is legal or not
    pub fn is_legal(&self, mv: &Move) -> bool {
        let mut cloned = self.clone();
        let bitboard = &mut cloned.pieces[mv.color][mv.piece];
        bitboard.set_0(mv.from);
        bitboard.set_1(mv.to);
        cloned.pieces[mv.color][mv.piece] = Bitboard::new(bitboard.num()) ;
        for piece in Piece::pieces() {
            let piece_bitboard = self.pieces[!mv.color][piece].num();
            if piece_bitboard != piece_bitboard & !(1u64 << mv.to) {
                cloned.pieces[!mv.color][piece] = Bitboard::new( piece_bitboard & !(1u64 << mv.to));
            }
        }

        !cloned.is_check(mv.color)
    }

    /// Makes a move
    pub fn make_move(&mut self, mv: &Move) -> Result<(), ChessError> {
        // NOTE: this check shouldn't be necessary
        // if !self.is_legal(mv) {
        //     return Err(ChessError::InvalidMove(format!("Invalid move: {:?}", mv)))
        // }
        //
        if self.state.is_finished() {
            return Err(ChessError::GameFinished)
        }
        if mv.piece == Pawn{
            self.repetition_history.clear();
        }

        let bitboard = &mut self.pieces[mv.color][mv.piece];
        match mv.flag{
            Flag::Default => {
                let bitboard = &mut self.pieces[mv.color][mv.piece];
                bitboard.set_0(mv.from);
                bitboard.set_1(mv.to);
                match mv.piece {
                    Pawn => self.halfmove_clock = 0,
                    _ => self.halfmove_clock += 1

                }
                self.en_passant = None;
            },
            Flag::LongPawnMove => {
                bitboard.set_0(mv.from);
                bitboard.set_1(mv.to);
                self.en_passant = if check_en_passant(mv.to, self, mv.color) {
                    match mv.color {
                        White => Some(mv.to - 8),
                        Black => Some(mv.to + 8)
                    }
                }else {
                    None
                };
                self.halfmove_clock = 0;
            }
            Flag::Capture(captured) => {
                bitboard.set_0(mv.from);
                bitboard.set_1(mv.to);
                let captured_bitboard = &mut self.pieces[!mv.color][captured];
                captured_bitboard.set_0(mv.to);
                self.en_passant = None;
                self.halfmove_clock = 0;
                self.repetition_history.clear();
            }
            Flag::EnPassant => {
                bitboard.set_0(mv.from);
                bitboard.set_1(mv.to);
                let enemy_pawns = &mut self.pieces[!mv.color][Pawn];
                match mv.color {
                    White => enemy_pawns.set_0(mv.to - 8),
                    Black => enemy_pawns.set_0(mv.to + 8),
                }
                self.en_passant = None;
                self.halfmove_clock = 0;
            }
            Flag::Promotion(prom) => {
                bitboard.set_0(mv.from);
                let promotion_bitboard = &mut self.pieces[mv.color][prom];
                promotion_bitboard.set_1(mv.to);
                self.en_passant = None;
                self.halfmove_clock = 0;
            }
            Flag::CapturePromotion(captured, prom) => {
                bitboard.set_0(mv.from);
                let captured_bitboard = &mut self.pieces[!mv.color][captured];
                captured_bitboard.set_0(mv.to);
                let promotion_bitboard = &mut self.pieces[mv.color][prom];
                promotion_bitboard.set_1(mv.to);
                self.en_passant = None;
                self.halfmove_clock = 0;
            }
            Flag::ShortCastle => {
                match mv.color {
                    White => {
                        self.pieces[White][Rook].set_0(7);
                        self.pieces[White][Rook].set_1(5);
                        self.pieces[White][King].set_0(4);
                        self.pieces[White][King].set_1(6);
                    }
                    Black => {
                        self.pieces[Black][Rook].set_0(63);
                        self.pieces[Black][Rook].set_1(61);
                        self.pieces[Black][King].set_0(60);
                        self.pieces[Black][King].set_1(62);
                    }
                }
                self.repetition_history.clear();
            }
            Flag::LongCastle => {
                match mv.color {
                    White => {
                        self.pieces[White][Rook].set_0(0);
                        self.pieces[White][Rook].set_1(3);
                        self.pieces[White][King].set_0(4);
                        self.pieces[White][King].set_1(2);
                    }
                    Black => {
                        self.pieces[Black][Rook].set_0(56);
                        self.pieces[Black][Rook].set_1(59);
                        self.pieces[Black][King].set_0(60);
                        self.pieces[Black][King].set_1(58);
                    }
                }
                self.repetition_history.clear();
            }
        }

        // deal with castling rights
        if mv.piece == King {
            match mv.color {
                White => {
                    self.castling_rights[White][KingSide] = false;
                    self.castling_rights[White][QueenSide] = false;
                }
                Black => {
                    self.castling_rights[Black][KingSide] = false;
                    self.castling_rights[Black][QueenSide] = false;
                }
            }
        }
        else if mv.piece == Rook {
            match mv.color {
                White => {
                    self.castling_rights[White][QueenSide] = self.castling_rights[White][QueenSide] && mv.from != 0;
                    self.castling_rights[White][KingSide] = self.castling_rights[White][KingSide] && mv.from != 7;
                }
                Black => {
                    self.castling_rights[Black][QueenSide] = self.castling_rights[Black][QueenSide] && mv.from != 56;
                    self.castling_rights[Black][KingSide] = self.castling_rights[Black][KingSide] && mv.from != 63;

                }
            }
        }

        self.turn = !self.turn;
        self.move_history.push(*mv);
        self.check = if self.is_check(White) {
            Some(White)
        } else if self.is_check(Black) {
            Some(Black)
        } else {
            None
        };
        self.repetition_history.push(self.get_hash());
        self.state = self.check_state();
        
        Ok(())
    }

    /// Returns whether the provided side is in check
    pub fn is_check(&self, color: Color) -> bool {
        let bitboard = self.pieces[color][King].num();
        let square = lsb_index(bitboard).unwrap();
        self.square_is_attacked(square, color)
    }

    pub fn square_is_attacked(&self, square: u32, color: Color) -> bool {
        let bitboard = 1u64 << square;
        let enemies = match color {
            White => self.pieces[Black],
            Black => self.pieces[White]
        };

        if bitboard & all_pawn_captures(enemies[5].num(), self, !color).num() != 0 {
            return true;
        }
        if knight_moves(square, self, color).num() & enemies[4].num() != 0 {
            return true;            
        }
        if bishop_moves(square, self, color).num() & enemies[3].num() != 0 {
            return true;            
        }
        if rook_moves(square, self, color).num() & enemies[2].num() != 0 {
            return true;            
        }
        if queen_moves(square, self, color).num() & enemies[1].num() != 0 {
            return true;            
        }
        if king_moves(square, self, color).num() & enemies[0].num() != 0 {
            return true;            
        }

        false
    }

    pub fn check_state(&self) -> GameState {
        if self.gen_moves().is_empty() {
            if self.is_check(self.turn) {
                GameState::checkmate(!self.turn)
            } else {
                GameState::stalemate()
            }
        }else if self.halfmove_clock == 100 {
                GameState::fifty_moves()
        }else{
            let hash = self.get_hash();
            let mut ctr = 0;
            for hash1 in self.repetition_history.iter() {
                if hash == *hash1 {
                    ctr += 1;
                };
                if ctr == 3 {
                    return GameState::three_reps()
                }
            };
            GameState::ongoing()
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hash = 0u64;
        if self.turn == Black { hash ^= *BLACK_MOVE_KEY; }
        if self.castling_rights[White][KingSide] { hash ^= CASTLING_KEYS[0][0]; }
        if self.castling_rights[White][QueenSide] { hash ^= CASTLING_KEYS[0][1]; }
        if self.castling_rights[Black][KingSide] { hash ^= CASTLING_KEYS[1][0]; }
        if self.castling_rights[Black][QueenSide] { hash ^= CASTLING_KEYS[1][1]; }

        (0..64).for_each(|square| {
            if let Some(piece) = self.find_piece(White, square) {
                hash ^= PIECE_KEYS[square as usize][0][piece as usize];
            }
            if let Some(piece) = self.find_piece(Black, square) {
                hash ^= PIECE_KEYS[square as usize][1][piece as usize];
            }
        });

        hash
        
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            pieces: [
                [
                    Bitboard::from(WK),
                    Bitboard::from(WQ),
                    Bitboard::from(WR),
                    Bitboard::from(WB),
                    Bitboard::from(WN),
                    Bitboard::from(WP),
                ],
                [
                    Bitboard::from(BK),
                    Bitboard::from(BQ),
                    Bitboard::from(BR),
                    Bitboard::from(BB),
                    Bitboard::from(BN),
                    Bitboard::from(BP),
                ],
            ],
            castling_rights: [[true; 2]; 2],
            turn: White,
            check: None,
            en_passant: None,
            move_history: Vec::new(),
            halfmove_clock: 0,
            state: GameState::Ongoing,
            repetition_history: Vec::new()
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for (i, mv) in self.move_history.iter().enumerate(){
            if i % 2 == 0 {
                write!(f, "{}. {} ", (i / 2) + 1, mv)?;
            }else{
                writeln!(f, "{}", mv)?;

            }
        }
        writeln!(f)?;

        let mut pieces: [char; 64] = [' '; 64];
        // for p in self.pieces[White][King] {
        //     pieces[p as usize] = 'K';
        // }
        // for p in self.pieces[White][Queen] {
        //     pieces[p as usize] = 'Q';
        // }
        // for p in self.pieces[White][Rook] {
        //     pieces[p as usize] = 'R';
        // }
        // for p in self.pieces[White][Bishop] {
        //     pieces[p as usize] = 'B';
        // }
        // for p in self.pieces[White][Knight] {
        //     pieces[p as usize] = 'N';
        // }
        // for p in self.pieces[White][Pawn] {
        //     pieces[p as usize] = 'P';
        // }
        // for p in self.pieces[Black][King] {
        //     pieces[p as usize] = 'k';
        // }
        // for p in self.pieces[Black][Queen] {
        //     pieces[p as usize] = 'q';
        // }
        // for p in self.pieces[Black][Rook] {
        //     pieces[p as usize] = 'r';
        // }
        // for p in self.pieces[Black][Bishop] {
        //     pieces[p as usize] = 'b';
        // }
        // for p in self.pieces[Black][Knight] {
        //     pieces[p as usize] = 'n';
        // }
        // for p in self.pieces[Black][Pawn] {
        //     pieces[p as usize] = 'p';
        // }

        for p in self.pieces[White][King] {
            pieces[p as usize] = Piece::symbol(King, White);
        }
        for p in self.pieces[White][Queen] {
            pieces[p as usize] = Piece::symbol(Queen, White);
        }
        for p in self.pieces[White][Rook] {
            pieces[p as usize] = Piece::symbol(Rook, White);
        }
        for p in self.pieces[White][Bishop] {
            pieces[p as usize] = Piece::symbol(Bishop, White);
        }
        for p in self.pieces[White][Knight] {
            pieces[p as usize] = Piece::symbol(Knight, White);
        }
        for p in self.pieces[White][Pawn] {
            pieces[p as usize] = Piece::symbol(Pawn, White);
        }
        for p in self.pieces[Black][King] {
            pieces[p as usize] = Piece::symbol(King, Black);
        }
        for p in self.pieces[Black][Queen] {
            pieces[p as usize] = Piece::symbol(Queen, Black);
        }
        for p in self.pieces[Black][Rook] {
            pieces[p as usize] = Piece::symbol(Rook, Black);
        }
        for p in self.pieces[Black][Bishop] {
            pieces[p as usize] = Piece::symbol(Bishop, Black);
        }
        for p in self.pieces[Black][Knight] {
            pieces[p as usize] = Piece::symbol(Knight, Black);
        }
        for p in self.pieces[Black][Pawn] {
            pieces[p as usize] = Piece::symbol(Pawn, Black);
        }
        for i in (0..8).rev() {
            writeln!(f, "+---+---+---+---+---+---+---+---+")?;
            for j in 0..8 {
                write!(f, "| {} ", pieces[i * 8 + j])?;
            }
            writeln!(f, "| {}", i + 1)?;
        }
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        writeln!(f, "  a   b   c   d   e   f   g   h  ")?;
        writeln!(f, "\nMove: {:?}", self.turn)?;
        if self.check.is_some() {
            match self.check.unwrap() {
                White => writeln!(f, "White checked!")?,
                Black => writeln!(f, "Black checked!")?,
            }
        }
        writeln!(f, "Halfmove clock: {}", self.halfmove_clock)?;
        if let Some(en_passant) = self.en_passant {
            writeln!(f, "En passant: {}", index_to_square(en_passant))?;
        };
        writeln!(f, "FEN: {}", self.to_fen())?;
        if self.state.is_finished() {
            writeln!(f, "\nGame finished: {}", self.state)?;
        }
        Ok(())
    }
}
