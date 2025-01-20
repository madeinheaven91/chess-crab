use std::{
    borrow::BorrowMut, cell::RefCell, io::{stdout, Write}, ops::Deref, rc::Rc
};

use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use shared::square_to_index;

pub mod bitboard;
pub mod errors;
pub mod game;
pub mod masks;
pub mod shared;
#[cfg(test)]
pub mod test;

struct State {
    game: Rc<RefCell<Game>>,
    message: Rc<RefCell<String>>,
}

fn main() -> anyhow::Result<()> {
    let game = Rc::new(RefCell::new(Game::default()));
    let message = Rc::new(RefCell::new(String::new()));
    let mut state = State {
        game,
        message: message.clone(),
    };

    execute!(stdout(), EnterAlternateScreen)?;

    main_loop(&mut state)?;

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn main_loop(state: &mut State) -> anyhow::Result<()> {
    loop {
        execute!(stdout(), Clear(ClearType::All))?;
        let game = state.game.borrow();
        let message = state.message.borrow();
        println!("{}\n{}", game, message);
        print!("[prompt]: ");
        stdout().flush()?;
        drop(message);
        // drop(game);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let cmd = buf.trim().split(" ").map(|s| s.trim()).collect::<Vec<_>>();
        let new_msg = match cmd[0] {
            "q" | "quit" | "exit" => break,
            "moves" => game
                .gen_moves()
                .iter()
                .fold(String::from("Available moves: "), |acc, m| {
                    acc + format!("{} ", m).as_str()
                }),
            "index" => {
                format!("Index: {}", square_to_index(cmd[1]))
            }
            "move" => {
                let mv = match game.parse_move(cmd[1]){
                    Err(_) => {
                        // TODO: proper error handling
                        continue;
                    }
                    Ok(mv) => mv,
                };

                if game.gen_moves().contains(&mv) {
                    drop(game);
                    let mut game_mut = state.game.deref().borrow_mut();
                    game_mut.make_move(&mv);
                }
                String::new()
            }
            _ => String::default(),
        };

        let message = state.message.borrow_mut();
        message.replace_with(|_| new_msg);
    }
    Ok(())
}
