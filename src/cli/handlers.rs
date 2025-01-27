use crate::shared::functions::square_to_index;

use std::{
    borrow::BorrowMut, ops::Deref}
;

use super::state::{Signal, State};

pub fn handle_cmd(state: &mut State, cmd: &Vec<&str>) -> Result<Signal, ()> {
    let mut cmd = cmd.to_vec();
    // Normalizing input length
    while cmd.len() != 5{
        cmd.push("");
    }

    let game = state.game.borrow();
    match cmd[0] {
        "q" | "quit" | "exit" => Ok(Signal::Exit),
        "moves" => {
            let msg = game.gen_moves()
                .iter()
                .fold(String::from("Available moves: "), |acc, m| {
                    acc + format!("{} ", m).as_str()
                });
            Ok(Signal::Message(msg))
        }
        "index" => {
            Ok(Signal::Message(format!("Index: {}", square_to_index(cmd[1]))))
        }
        "move" => {
            let mv = match game.parse_move(cmd[1]) {
                Err(_) => {
                    let message = state.message.borrow_mut();
                    message.replace_with(|_| "Incorrect move!".to_string());
                    return Ok(Signal::Continue)
                }
                Ok(mv) => mv,
            };

            if game.gen_moves().contains(&mv) {
                drop(game);
                let mut game_mut = state.game.deref().borrow_mut();
                game_mut.make_move(&mv);
            }
            Ok(Signal::Message(String::new()))
        }
        _ => Ok(Signal::Continue),
    }
}
