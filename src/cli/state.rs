use std::{cell::RefCell, fmt::Display, io::{stdout, Write}, ops::Deref, rc::Rc};
use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::{cli::handlers::handle_cmd, game::structs::Game};
use crate::shared::functions::square_to_index;
use std::
    borrow::BorrowMut
;

pub struct State {
    pub game: Rc<RefCell<Game>>,
    pub message: Rc<RefCell<String>>,
}

impl State{
    pub fn new(game: Game) -> Self{
        Self{
            game: Rc::new(RefCell::new(game)),
            message: Rc::new(RefCell::new(String::new())),
        }
    }
    pub fn rewrite(&mut self, new_msg: String){
        let message = self.message.borrow_mut();
        message.replace_with(|_| new_msg);
    }

    // pub fn game_mut_ref(&mut self) -> &mut Rc<RefCell<Game>>{
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
    Message(String)
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
        let mut cmd = buf.trim().split(" ").map(|s| s.trim()).collect::<Vec<_>>();

        let new_msg = match handle_cmd(state, &cmd){
            Ok(signal) => match signal{
                Signal::Exit => break,
                Signal::Continue => continue,
                Signal::Message(msg) => msg,
            }
            Err(_) => break,
        };

        state.rewrite(new_msg);
        //
        // let message = state.message.borrow_mut();
        // message.replace_with(|_| new_msg);
    }
    Ok(())
}


