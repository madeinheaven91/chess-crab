use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ChessError{
    InvalidMove(String)
}

impl Display for ChessError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ChessError::InvalidMove(m) => write!(f, "{}", m)
        }
    }
}

impl Error for ChessError{ }
