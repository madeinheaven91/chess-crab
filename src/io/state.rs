use std::{cell::RefCell, fmt::Display, io::{stdout, Write}, rc::Rc};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use crate::{io::cmd::{handle_cmd, handle_debug_cmd}, game::structs::board::Board};
use std::
    borrow::BorrowMut
;

pub struct State {
    pub game: Rc<RefCell<Board>>,
    pub message: Rc<RefCell<String>>,
    pub debug: bool
}

impl State{
    pub fn new(game: Board) -> Self{
        Self{
            game: Rc::new(RefCell::new(game)),
            message: Rc::new(RefCell::new(String::new())),
            debug: std::env::var("DEBUG").is_ok()
        }
    }
    pub fn rewrite(&mut self, new_msg: String){
        let message = self.message.borrow_mut();
        message.replace_with(|_| new_msg);
    }

    pub fn update_game(&mut self, new_game: Box<Board>){
        let game = self.game.borrow_mut();
        game.replace_with(|_| *new_game);
    }

    // pub fn game_mut_ref(&mut self) -> &mut Rc<RefCell<Board>>{
    //     self.game.borrow_mut()
    // }
    //
    // pub fn message_mut_ref(&mut self) -> &mut Rc<RefCell<String>>{
    //     self.message.borrow_mut()
    // }
}

impl Display for State{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let game = self.game.borrow();
        let message = self.message.borrow();
        write!(f, "{}\n{}", game, message)
    }
}

pub enum Signal {
    Exit,
    Continue,
    Message(String),
    Board(Box<Board>)
}

pub fn main_loop(state: &mut State) -> anyhow::Result<()> {
    loop {
        execute!(stdout(), Clear(ClearType::All))?;
        let message = state.message.borrow();
        println!("{}", state);
        print!("[prompt]: ");
        stdout().flush()?;
        drop(message);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let cmd = buf.trim().split(" ").map(|s| s.trim()).collect::<Vec<_>>();

        let res = match state.debug {
            false => handle_cmd(state, &cmd),
            true => handle_debug_cmd(state, &cmd)
        };

        match res{
            Ok(signal) => match signal{
                Signal::Exit => break,
                Signal::Continue => continue,
                Signal::Message(msg) => state.rewrite(msg),
                Signal::Board(game) => {
                    state.update_game(game);
                    state.rewrite(String::new());
                },
            }
            Err(_) => break,
        };
    }
    Ok(())
}
