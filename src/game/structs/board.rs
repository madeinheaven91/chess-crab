use crate::{game::moves::move_struct::Move, shared::{consts::*, errors::ChessError, functions::{index_to_square, lsb, lsb_index, msb, square_to_index}, statics::zobrist::{BLACK_MOVE_KEY, CASTLING_KEYS, PIECE_KEYS}}};

use super::{bitboard::Bitboard, color::{Color, Castling}, game_state::GameState, piece::Piece};

use std::{cell::RefCell, fmt::Display, ops::AddAssign, rc::Rc};
use Color::*;
use Piece::*;
use Castling::*;

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub turn: Color,
    pub castling_rights:[[bool; 2]; 2],
    pub en_passant: Option<u32>,
    // pub move_history: Vec<Move>,
    pub halfmove_clock: u8,
    pub repetition_history: Vec<u64>,

    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub all_pieces: Bitboard,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            pieces: [[Bitboard::empty(); 6]; 2],
            turn: White,
            castling_rights: [[false; 2]; 2],
            en_passant: None,
            // move_history: Vec::new(),
            halfmove_clock: 0,
            repetition_history: Vec::new(),

            white_pieces: Bitboard::empty(),
            black_pieces: Bitboard::empty(),
            all_pieces: Bitboard::empty()
        }
    }

    fn black_pieces(&self) -> Bitboard {
        self.pieces[Black][King]
        | self.pieces[Black][Queen]
        | self.pieces[Black][Rook]
        | self.pieces[Black][Bishop]
        | self.pieces[Black][Knight]
        | self.pieces[Black][Pawn]
    }

    fn white_pieces(&self) -> Bitboard {
        self.pieces[White][King]
        | self.pieces[White][Queen]
        | self.pieces[White][Rook]
        | self.pieces[White][Bishop]
        | self.pieces[White][Knight]
        | self.pieces[White][Pawn]
    }

    pub fn update_pieces(&mut self) {
        self.white_pieces = self.white_pieces();
        self.black_pieces = self.black_pieces();
        self.all_pieces = self.white_pieces | self.black_pieces;
    }

    pub fn enemies(&self, color: Color) -> Bitboard {
        match color {
            White => self.black_pieces,
            Black => self.white_pieces,
        }
    }

    pub fn friends(&self, color: Color) -> Bitboard {
        match color {
            White => self.white_pieces,
            Black => self.black_pieces,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Board, ChessError> {
        let mut res = Board::empty();
        let elements = fen.split(" ").collect::<Vec<&str>>();
        // 1 - board
        // 2 - turn
        // 3 - castling
        // 4 - en passant
        // 5 - halfmove clock
        // 6 - N of full moves

        // Parse board
        for (char, i) in elements[0]
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
            .zip(0..64u32)
        {
            if char.is_ascii_alphabetic() {
                match char {
                    'P' => res.pieces[White][Pawn].set_1(i),
                    'N' => res.pieces[White][Knight].set_1(i),
                    'B' => res.pieces[White][Bishop].set_1(i),
                    'R' => res.pieces[White][Rook].set_1(i),
                    'Q' => res.pieces[White][Queen].set_1(i),
                    'K' => res.pieces[White][King].set_1(i),
                    'p' => res.pieces[Black][Pawn].set_1(i),
                    'n' => res.pieces[Black][Knight].set_1(i),
                    'b' => res.pieces[Black][Bishop].set_1(i),
                    'r' => res.pieces[Black][Rook].set_1(i),
                    'q' => res.pieces[Black][Queen].set_1(i),
                    'k' => res.pieces[Black][King].set_1(i),
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
        let halfmove_clock = match elements[4] {
            "" => 0,
            _ => match elements[4].parse::<u8>() {
                Ok(val) => val,
                Err(_) => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid halfmove clock: {}", elements[4])))
        }};
        res.halfmove_clock = halfmove_clock;

        // Parse move count
        // NOTE: idk what to do with it
        let _halfmove_clock = match elements[5] {
            "" => 0,
            _ => match elements[5].parse::<u8>() {
                Ok(val) => val,
                Err(_) => return Err(ChessError::FENParseError(fen.to_string(), format!("Invalid halfmove clock: {}", elements[4])))
        }};

        res.update_pieces();
        if !res.is_valid() {
            return Err(ChessError::InvalidPosition)
        }
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
        for rank in (0..8).rev(){
            for file in 0..8{
                if let Some((color, piece)) = self.find_piece(rank * 8 + file){
                    if !empty_counter.borrow().eq(&0){
                        board_string.push_str(&empty_counter.borrow().to_string());
                        empty_counter.replace_with(|_| 0);
                    }
                    match color {
                        White => board_string.push(piece.char()),
                        Black => board_string.push(piece.char().to_ascii_lowercase()),
                    }
                }else{
                    empty_counter.borrow_mut().add_assign(1);
                }
            }
            if !empty_counter.borrow().eq(&0){
                board_string.push_str(&empty_counter.borrow().to_string());
            }
            empty_counter.replace_with(|_| 0);
            board_string.push('/');
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
            true => {
                let mut res = String::new();
                if self.castling_rights[White][KingSide] { res.push('K') };
                if self.castling_rights[White][QueenSide] { res.push('Q') };
                if self.castling_rights[Black][KingSide] { res.push('k') };
                if self.castling_rights[Black][QueenSide] { res.push('q') };
                res
            }
        };

        res[3] = match self.en_passant {
            Some(val) => index_to_square(val).to_string(),
            None => "-".to_string()
        };

        res[4] = self.halfmove_clock.to_string();

        // res[5] = (self.move_history.len() / 2 + 1).to_string();

        res.join(" ")

    }
    
    /// Finds a piece at a given index and returns its color and type.
    /// If there is no piece, returns `None`
    pub fn find_piece(&self, index: u32) -> Option<(Color, Piece)>{
        for (color, pieces) in self.pieces.iter().enumerate(){
            for (piece_i, piece) in pieces.iter().enumerate(){
                if piece.num() & (1u64 << index) != 0{
                    return Some((Color::from(color), Piece::from(piece_i)))
                }
            }
        };
        None
    }

    /// Parses an algebraically notated move into `Move`.
    /// Returns Err if the move string is invalid
    pub fn parse_move(&self, mv: &str) -> Result<Move, ChessError> {
        match mv.len() {
            4 => {
                if mv == "0000" {
                    return Ok(Move::null())
                };
                let (from, to) = mv.split_at(2);
                let from = square_to_index(from).unwrap();
                let to = square_to_index(to).unwrap();

                if let Some((color, piece)) = self.find_piece(from) {
                    Ok(Move::new(self, from, to, piece, color))
                }else{
                    Err(ChessError::InvalidMove(format!("No pieces can make move: {:?}", mv)))
                }
            }
            5 => {
                let chars = mv.chars().collect::<Vec<char>>();
                let from = square_to_index(&chars[0..2].iter().collect::<String>()).unwrap();
                let to = square_to_index(&chars[2..4].iter().collect::<String>()).unwrap();
                let promotion = match chars[4] {
                    'q' => Queen,
                    'r' => Rook,
                    'b' => Bishop,
                    'n' => Knight,
                    _ => return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv))),
                };

                if let Some((color, piece)) = self.find_piece(from) {
                    if piece != Pawn {
                        return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv)));
                    }
                    Ok(Move::promotion(self, from, to, color, promotion))
                }else{
                    Err(ChessError::InvalidMove(format!("No pieces can make move: {:?}", mv)))
                }
            }
            _ => Err(ChessError::InvalidMove(format!("Invalid move: {}", mv))),
        }
    }

    /// Returns the color of the checked side in a current position.
    /// Returns `None` if no side is in check.
    pub fn is_check(&self) -> Option<Color> {
        if self.square_is_attacked(lsb_index(self.pieces[White][King]).unwrap_or(0), Black) {
            Some(White)
        }else if self.square_is_attacked(lsb_index(self.pieces[Black][King]).unwrap_or(0), White) {
            Some(Black)
        }else {
            None
        }
    }

    pub fn check_state(&self) -> GameState {
        if self.halfmove_clock == 100 {
                GameState::Draw
        }else if self.gen_legal_moves().is_empty() {
            if let Some(color) = self.is_check() {
                GameState::Win(!color)
            } else {
                GameState::Draw
            }
        }else{
            let hash = self.get_hash();
            let mut ctr = 0;
            for hash1 in self.repetition_history.iter() {
                if hash == *hash1 {
                    ctr += 1;
                };
            };
            if ctr >= 3 {
                return GameState::Draw
            }
            GameState::Ongoing
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
            if let Some((color, piece)) = self.find_piece(square) {
                hash ^= PIECE_KEYS[square as usize][color as usize][piece as usize];
            }
        });

        hash
    }

    pub fn is_valid(&self) -> bool {
        // Check if opposite of side isnt in check
        if let Some(color) = self.is_check() {
            if color != self.turn {
                return false
            }
        };

        // Check if only 1 king
        if lsb(self.pieces[White][King]) != msb(self.pieces[White][King]) && self.pieces[White][King] != Bitboard::empty(){
            return false
        }
        if lsb(self.pieces[Black][King]) != msb(self.pieces[Black][King]) && self.pieces[Black][King] != Bitboard::empty(){
            return false
        }

        // Check if no pawns on 1 and 8 rank
        if (self.pieces[White][Pawn] | self.pieces[Black][Pawn]) & Bitboard::from(RANK_1 | RANK_8) != 0 {
            return false
        }

        true
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
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
            en_passant: None,
            // move_history: Vec::new(),
            halfmove_clock: 0,
            repetition_history: Vec::new(),

            white_pieces: Bitboard::from(WK) | Bitboard::from(WQ) | Bitboard::from(WR) | Bitboard::from(WB) | Bitboard::from(WN) | Bitboard::from(WP),
            black_pieces: Bitboard::from(BK) | Bitboard::from(BQ) | Bitboard::from(BR) | Bitboard::from(BB) | Bitboard::from(BN) | Bitboard::from(BP),
            all_pieces: Bitboard::from(BK) | Bitboard::from(BQ) | Bitboard::from(BR) | Bitboard::from(BB) | Bitboard::from(BN) | Bitboard::from(BP) | Bitboard::from(WK) | Bitboard::from(WQ) | Bitboard::from(WR) | Bitboard::from(WB) | Bitboard::from(WN) | Bitboard::from(WP),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        // for (i, mv) in self.move_history.iter().enumerate(){
        //     if i % 2 == 0 {
        //         write!(f, "{}. {} ", (i / 2) + 1, mv)?;
        //     }else{
        //         writeln!(f, "{}", mv)?;
        //
        //     }
        // }
        // writeln!(f)?;

        let mut pieces: [char; 64] = [' '; 64];

        for piece in Piece::pieces() {
            for color in Color::colors() {
                for i in self.pieces[color][piece] {
                    pieces[i as usize] = Piece::symbol(piece, color);
                }
            }
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

        if let Some(color) = self.is_check() {
            writeln!(f, "{} checked!", color)?
        }
        writeln!(f, "FEN: {}", self.to_fen())?;
        if self.check_state().is_finished() {
            writeln!(f, "\nGame finished: {}", self.check_state())?;
        }
        Ok(())
    }
}

