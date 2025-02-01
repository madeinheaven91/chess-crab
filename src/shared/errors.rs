use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ChessError{
    SquareParseError(String),
    InvalidMove(String),
    FENParseError(String, String)
}

impl Display for ChessError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ChessError::InvalidMove(m) => write!(f, "{}", m),
            ChessError::SquareParseError(sq) => write!(f, "Couldn't parse square: {:?}", sq),
            ChessError::FENParseError(fen, details) => write!(f, "Couldn't parse FEN string.\nInput: {}\nDetails: {}", fen, details)
        }
    }
}

impl Error for ChessError{ }
