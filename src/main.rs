use std::
    io::stdout
;

use io::state::{main_loop, State};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use game::structs::board::Board;
use shared::statics::init_statics;

pub mod game;
pub mod shared;
pub mod io;
#[cfg(test)]
pub mod test;


fn main() -> anyhow::Result<()> {
    init_statics();

    let game = Board::default();
    let mut state = State::new(game);

    execute!(stdout(), EnterAlternateScreen)?;

    main_loop(&mut state)?;

    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

