use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ChessError{
    SquareParseError(String),
    InvalidMove(String),
    FENParseError(String, String),
    GameFinished,
    InvalidPosition
}

impl Display for ChessError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ChessError::InvalidMove(m) => write!(f, "{}", m),
            ChessError::SquareParseError(sq) => write!(f, "Couldn't parse square: {:?}", sq),
            ChessError::FENParseError(fen, details) => write!(f, "Couldn't parse FEN string.\nInput: {}\nDetails: {}", fen, details),
            ChessError::GameFinished => write!(f, "Couldn't make a move, game is finished."),
            ChessError::InvalidPosition => write!(f, "Invalid position")
        }
    }
}

impl Error for ChessError{ }
