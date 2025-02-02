use std::fmt::Display;
use super::color::Color;

#[derive(Clone, Debug)]
pub enum GameState{
    Ongoing,
    Finished(GameResult)
}

#[derive(Clone, Debug)]
pub enum GameResult {
    Checkmate(Color),
    Stalemate,
    ThreeMoveRep,
    FiftyMoveRule
}

impl GameState{
    pub fn checkmate(color: Color) -> GameState {
        GameState::Finished(GameResult::Checkmate(color))
    }

    pub fn stalemate() -> GameState {
        GameState::Finished(GameResult::Stalemate)
    }

    pub fn ongoing() -> GameState {
        GameState::Ongoing
    }

    pub fn fifty_moves() -> GameState {
        GameState::Finished(GameResult::FiftyMoveRule)
    }

    pub fn three_reps() -> GameState {
        GameState::Finished(GameResult::ThreeMoveRep)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, GameState::Finished(_))
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Ongoing => write!(f, "Ongoing"),
            GameState::Finished(result) => write!(f, "{}", result)
        }
    }
}


impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Checkmate(color) => write!(f, "{} wins by checkmate", color),
            GameResult::Stalemate => write!(f, "Draw by stalemate"),
            GameResult::ThreeMoveRep => write!(f, "Draw by three move repetition"),
            GameResult::FiftyMoveRule => write!(f, "Draw by fifty move rule")
        }
    }
}
