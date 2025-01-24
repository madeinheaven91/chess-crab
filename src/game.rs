use crate::{
    bitboard::{Bitboard, Piece},
    errors::ChessError,
    masks::{
        bishop_attacks, king_attacks, knight_attacks, pawn_attacks, pawn_captures, queen_attacks, rook_attacks
    },
    shared::*,
};
use std::{fmt::Display, ops::Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color{
    pub fn colors() -> [Color; 2]{
        [Color::White, Color::Black]
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    start: u32,
    end: u32,
    piece: Piece,
    color: Color,
    promotion: Option<Piece>
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prom = match self.promotion{
            Some(p) => p.to_string().to_lowercase(),
            None => "".to_string()
        };
        write!(
            f,
            "{}{}{}",
            index_to_square(self.start),
            index_to_square(self.end),
            prom
        )
    }
}


impl Move {
    pub fn new(start: u32, end: u32, piece: Piece, color: Color, promotion: Option<Piece>) -> Self{
        Move {
            start,
            end,
            piece,
            color,
            promotion
        }
    }
}


#[derive(Clone)]
pub struct Game {
    pub wk: Bitboard,
    pub wq: Bitboard,
    pub wr: Bitboard,
    pub wb: Bitboard,
    pub wn: Bitboard,
    pub wp: Bitboard,
    pub bk: Bitboard,
    pub bq: Bitboard,
    pub br: Bitboard,
    pub bb: Bitboard,
    pub bn: Bitboard,
    pub bp: Bitboard,
    pub turn: Color,
    pub check: Option<Color>,
    pub white_oo: bool,
    pub white_ooo: bool,
    pub black_oo: bool,
    pub black_ooo: bool,
    pub en_passant: Option<Bitboard>,
    pub halfmoves: Vec<Move>
}

impl Game {
    pub fn black_pieces(&self) -> Bitboard {
        Bitboard::from(
            self.bk.num()
                | self.bq.num()
                | self.br.num()
                | self.bb.num()
                | self.bn.num()
                | self.bp.num(),
        )
    }
    pub fn white_pieces(&self) -> Bitboard {
        Bitboard::from(
            self.wk.num()
                | self.wq.num()
                | self.wr.num()
                | self.wb.num()
                | self.wn.num()
                | self.wp.num(),
        )
    }
    pub fn all_pieces(&self) -> Bitboard {
        self.white_pieces() | self.black_pieces()
    }
    pub fn opposite_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.black_pieces(),
            Color::Black => self.white_pieces(),
        }
    }

    pub fn own_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    pub fn to_fen(&self) -> String {
        todo!()
    }

    pub fn get_bitboard(&self, piece: Piece, color: Color) -> Bitboard {
        use Color::*;
        use Piece::*;
        match (color, piece) {
            (White, King) => self.wk,
            (White, Queen) => self.wq,
            (White, Rook) => self.wr,
            (White, Bishop) => self.wb,
            (White, Knight) => self.wn,
            (White, Pawn) => self.wp,
            (Black, King) => self.bk,
            (Black, Queen) => self.bq,
            (Black, Rook) => self.br,
            (Black, Bishop) => self.bb,
            (Black, Knight) => self.bn,
            (Black, Pawn) => self.bp,
        }
    }

    pub fn set_bitboard(&mut self, bitboard: Bitboard, piece: Piece, color: Color) {
        use Color::*;
        use Piece::*;
        match (color, piece) {
            (White, King) => self.wk = bitboard,
            (White, Queen) => self.wq = bitboard,
            (White, Rook) => self.wr = bitboard,
            (White, Bishop) => self.wb = bitboard,
            (White, Knight) => self.wn = bitboard,
            (White, Pawn) => self.wp = bitboard,
            (Black, King) => self.bk = bitboard,
            (Black, Queen) => self.bq = bitboard,
            (Black, Rook) => self.br = bitboard,
            (Black, Bishop) => self.bb = bitboard,
            (Black, Knight) => self.bn = bitboard,
            (Black, Pawn) => self.bp = bitboard,
        }
    }

    /// Generates a list of all possible moves in a current position
    pub fn gen_moves(&self) -> Vec<Move> {
        use Color::*;
        use Piece::*;
        let mut res = vec![];
        match self.turn {
            Color::White => {
                for p in self.wk {
                    for m in king_attacks(p, self, Color::White) {
                        let mv = Move::new(p, m, King, White, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }

                    }
                }
                for p in self.wq {
                    for m in queen_attacks(p, self, Color::White) {
                        let mv = Move::new(p, m, Queen, White, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.wr {
                    for m in rook_attacks(p, self, Color::White) {
                        let mv = Move::new(p, m, Rook, White, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.wb {
                    for m in bishop_attacks(p, self, Color::White) {
                        let mv = Move::new(p, m, Bishop, White, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.wn {
                    for m in knight_attacks(p, self, Color::White) {
                        let mv = Move::new(p, m, Knight, White, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.wp {
                    for m in pawn_attacks(p, self, Color::White) {
                        if m / 8 == 7 {
                            for piece in Piece::promotable(){
                                let mv = Move::new(p, m, Pawn, White, Some(piece));
                                if self.simulate_move(&mv) {
                                    res.push(mv);
                                }
                            }
                        }else{
                            let mv = Move::new(p, m, Pawn, White, None);
                            if self.simulate_move(&mv) {
                                res.push(mv);
                            }

                        }
                    }
                }
            }
            Color::Black => {
                for p in self.bk {
                    for m in king_attacks(p, self, Color::Black) {
                        let mv = Move::new(p, m, King, Black, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.bq {
                    for m in queen_attacks(p, self, Color::Black) {
                        let mv = Move::new(p, m, Queen, Black, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.br {
                    for m in rook_attacks(p, self, Color::Black) {
                        let mv = Move::new(p, m, Rook, Black, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.bb {
                    for m in bishop_attacks(p, self, Color::Black) {
                        let mv = Move::new(p, m, Bishop, Black, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.bn {
                    for m in knight_attacks(p, self, Color::Black) {
                        let mv = Move::new(p, m, Knight, Black, None);
                        if self.simulate_move(&mv) {
                            res.push(mv);
                        }
                    }
                }
                for p in self.bp {
                    for m in pawn_attacks(p, self, Color::Black) {
                        if m / 8 == 0 {
                            for piece in Piece::promotable(){
                                let mv = Move::new(p, m, Pawn, Black, Some(piece));
                                if self.simulate_move(&mv) {
                                    res.push(mv);
                                }
                            }
                        }else{
                            let mv = Move::new(p, m, Pawn, Black, None);
                            if self.simulate_move(&mv) {
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
    // TODO: pawn promotions (e7e8q)
    pub fn parse_move(&self, mv: &str) -> Result<Move, ChessError> {
        use Color::*;
        use Piece::*;
        if mv.len() != 4 && mv.len() != 5 {
            return Err(ChessError::InvalidMove(format!("Invalid move: {}", mv)));
        };

        let (start, end) = mv.split_at(2);
        let start = square_to_index(start);
        let end = square_to_index(end);

        // Find piece
        let (piece, color) = {
            if self.wk.num() & (1 << start) != 0 {
                (King, White)
            } else if self.wq.num() & (1 << start) != 0 {
                (Queen, White)
            } else if self.wr.num() & (1 << start) != 0 {
                (Rook, White)
            } else if self.wb.num() & (1 << start) != 0 {
                (Bishop, White)
            } else if self.wn.num() & (1 << start) != 0 {
                (Knight, White)
            } else if self.wp.num() & (1 << start) != 0 {
                (Pawn, White)
            } else if self.bk.num() & (1 << start) != 0 {
                (King, Black)
            } else if self.bq.num() & (1 << start) != 0 {
                (Queen, Black)
            } else if self.br.num() & (1 << start) != 0 {
                (Rook, Black)
            } else if self.bb.num() & (1 << start) != 0 {
                (Bishop, Black)
            } else if self.bn.num() & (1 << start) != 0 {
                (Knight, Black)
            } else if self.bp.num() & (1 << start) != 0 {
                (Pawn, Black)
            } else {
                return Err(ChessError::InvalidMove(format!(
                    "No pieces can make move: {}",
                    mv
                )));
            }
        };

        Ok(Move {
            start,
            end,
            piece,
            color,
            promotion: None
        })
    }

    /// Simulates a halfmove and returns whether it is legal or not
    pub fn simulate_move(&self, mv: &Move) -> bool{
        let mut cloned = self.clone();
        let mut bitboard = cloned.get_bitboard(mv.piece, mv.color).num();
        bitboard &= !(1u64 << mv.start);
        bitboard |= 1u64 << mv.end;
        cloned.set_bitboard(
            Bitboard::new(bitboard, Some(mv.piece), Some(mv.color)),
            mv.piece,
            mv.color,
        );
        for piece in Piece::pieces(){
            let piece_bitboard = self.get_bitboard(piece, !mv.color).num();
            if piece_bitboard != piece_bitboard & !(1u64 << mv.end) {
                cloned.set_bitboard(Bitboard::new(piece_bitboard & !(1u64 << mv.end), Some(piece), Some(!mv.color)), piece, !mv.color);
            }
        }

        let king = Bitboard::lsb_index(cloned.get_bitboard(Piece::King, mv.color).num()).unwrap();
        
        !cloned.is_check(mv.color)
    }

    pub fn make_move(&mut self, mv: &Move) {
        if !self.simulate_move(mv){
            panic!("Illegal move")
        }
        let mut bitboard = self.get_bitboard(mv.piece, mv.color).num();
        bitboard &= !(1u64 << mv.start);
        match mv.promotion {
            None => {
                bitboard |= 1u64 << mv.end;
                self.set_bitboard(
                    Bitboard::new(bitboard, Some(mv.piece), Some(mv.color)),
                    mv.piece,
                    mv.color,
                );
            }

            Some(promotion) => {
                let mut promotion_bitboard = self.get_bitboard(promotion, mv.color).num();
                promotion_bitboard |= 1u64 << mv.end;
                self.set_bitboard(
                    Bitboard::new(bitboard, Some(mv.piece), Some(mv.color)),
                    mv.piece,
                    mv.color,
                );
                self.set_bitboard(
                    Bitboard::new(promotion_bitboard, Some(promotion), Some(mv.color)),
                    promotion,
                    mv.color,
                );
            }

        };
        for piece in Piece::pieces(){
            let piece_bitboard = self.get_bitboard(piece, !mv.color).num();
            if piece_bitboard != piece_bitboard & !(1u64 << mv.end) {
                self.set_bitboard(Bitboard::new(piece_bitboard & !(1u64 << mv.end), Some(piece), Some(!mv.color)), piece, !mv.color);
            }
        }
        self.turn = self.turn.not();
        self.halfmoves.push(*mv);
        self.check = if self.is_check(Color::White) {
            Some(Color::White)
                }else if self.is_check(Color::Black){
            Some(Color::Black)
                } else {
                None
            }
    }

    pub fn is_check(&self, color: Color) -> bool {
        let bitboard = self.get_bitboard(Piece::King, color).num();
        let enemies = match color{
            Color::White => [self.bp, self.bn, self.bb, self.br, self.bq, self.bk ],
            Color::Black => [self.wp, self.wn, self.wb, self.wr, self.wq, self.wk ],
        };

        for pawn in enemies[0] {
            if bitboard & pawn_captures(pawn, self, !color).num() != 0 {
                return true;
            }
        }
        for knight in enemies[1] {
            if bitboard & knight_attacks(knight, self, !color).num() != 0 {
                return true;
            }
        }
        for bishop in enemies[2] {
            if bitboard & bishop_attacks(bishop, self, !color).num() != 0 {
                return true;
            }
        }
        for rook in enemies[3] {
            if bitboard & rook_attacks(rook, self, !color).num() != 0 {
                return true;
            }
        }
        for queen in enemies[4] {
            if bitboard & queen_attacks(queen, self, !color).num() != 0 {
                return true;
            }
        }
        for king in enemies[5] {
            if bitboard & king_attacks(king, self, !color).num() != 0 {
                return true;
            }
        }
        false
    }
}

impl Default for Game {
    fn default() -> Self {
        use Color::*;
        use Piece::*;
        Game {
            wk: Bitboard::new(WK, Some(King), Some(White)),
            wq: Bitboard::new(WQ, Some(King), Some(White)),
            wr: Bitboard::new(WR, Some(King), Some(White)),
            wb: Bitboard::new(WB, Some(King), Some(White)),
            wn: Bitboard::new(WN, Some(King), Some(White)),
            wp: Bitboard::new(WP, Some(King), Some(White)),
            bk: Bitboard::new(BK, Some(King), Some(White)),
            bq: Bitboard::new(BQ, Some(King), Some(White)),
            br: Bitboard::new(BR, Some(King), Some(White)),
            bb: Bitboard::new(BB, Some(King), Some(White)),
            bn: Bitboard::new(BN, Some(King), Some(White)),
            bp: Bitboard::new(BP, Some(King), Some(White)),
            turn: Color::White,
            check: None,
            white_oo: true,
            white_ooo: true,
            black_oo: true,
            black_ooo: true,
            en_passant: None,
            halfmoves: Vec::new(),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { let mut pieces: [char; 64] = [' '; 64];
        for p in self.wk {
            pieces[p as usize] = 'K';
        }
        for p in self.wq {
            pieces[p as usize] = 'Q';
        }
        for p in self.wr {
            pieces[p as usize] = 'R';
        }
        for p in self.wb {
            pieces[p as usize] = 'B';
        }
        for p in self.wn {
            pieces[p as usize] = 'N';
        }
        for p in self.wp {
            pieces[p as usize] = 'P';
        }
        for p in self.bk {
            pieces[p as usize] = 'k';
        }
        for p in self.bq {
            pieces[p as usize] = 'q';
        }
        for p in self.br {
            pieces[p as usize] = 'r';
        }
        for p in self.bb {
            pieces[p as usize] = 'b';
        }
        for p in self.bn {
            pieces[p as usize] = 'n';
        }
        for p in self.bp {
            pieces[p as usize] = 'p';
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
        if self.check.is_some(){
            match self.check.unwrap() {
                Color::White => writeln!(f, "White checked!")?,
                Color::Black => writeln!(f, "Black checked!")?,
            }
        }
        Ok(())
    }
}

impl Game{
    pub fn empty() -> Game{
        use Color::*;
        use Piece::*;
        Game {
            wk: Bitboard::new(0, Some(King), Some(White)),
            wq: Bitboard::new(0, Some(King), Some(White)),
            wr: Bitboard::new(0, Some(King), Some(White)),
            wb: Bitboard::new(0, Some(King), Some(White)),
            wn: Bitboard::new(0, Some(King), Some(White)),
            wp: Bitboard::new(0, Some(King), Some(White)),
            bk: Bitboard::new(0, Some(King), Some(White)),
            bq: Bitboard::new(0, Some(King), Some(White)),
            br: Bitboard::new(0, Some(King), Some(White)),
            bb: Bitboard::new(0, Some(King), Some(White)),
            bn: Bitboard::new(0, Some(King), Some(White)),
            bp: Bitboard::new(0, Some(King), Some(White)),
            turn: Color::White,
            check: None,
            white_oo: true,
            white_ooo: true,
            black_oo: true,
            black_ooo: true,
            en_passant: None,
            halfmoves: Vec::new(),
        }
    }
}
