use std::fmt::Display;
use super::color::Color;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum GameState{
    Ongoing,
    Win(Color),
    Draw
}

impl GameState{
    pub fn is_finished(&self) -> bool {
        matches!(self, GameState::Win(_) | GameState::Draw)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Ongoing => write!(f, "Ongoing"),
            GameState::Draw => write!(f, "Draw"),
            GameState::Win(color) => write!(f, "{color} won!"),
        }
    }
}
