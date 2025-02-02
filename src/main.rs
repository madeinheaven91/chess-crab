use std::
    io::stdout
;

use cli::state::{main_loop, State};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use game::structs::game::Game;

pub mod game;
pub mod shared;
pub mod cli;
#[cfg(test)]
pub mod test;


fn main() -> anyhow::Result<()> {
    let game = Game::default();
    let mut state = State::new(game);

    execute!(stdout(), EnterAlternateScreen)?;

    main_loop(&mut state)?;

    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

