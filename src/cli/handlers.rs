use crate::{game::structs::game::Game, shared::functions::square_to_index};

use std::{
    borrow::{BorrowMut}, ops::Deref}
;

use super::state::{Signal, State};

pub fn handle_cmd(state: &mut State, cmd: &Vec<&str>) -> Result<Signal, ()> {
    let mut cmd = cmd.to_vec();
    // Normalizing input length
    while cmd.len() < 5{
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
            Ok(Signal::Message(format!("Index: {}", square_to_index(cmd[1]).unwrap())))
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
                let move_res = game_mut.make_move(&mv);
                match move_res {
                    Ok(_) =>  Ok(Signal::Message(String::new())),
                    Err(err) =>  Ok(Signal::Message(err.to_string()))
                }
            }else{
                 Ok(Signal::Message("Invalid move!".to_string()))
            }
        }
        "position" => {
            match handle_position(cmd) {
                Ok(s) => Ok(s),
                Err(s) => Ok(s)
            }
        }
        "hide" => {
            state.show_board = false;
            Ok(Signal::Continue)
        }
        "show" => {
            state.show_board = true;
            Ok(Signal::Continue)
        }
        "random" => {
            drop(game);
            let mut game_mut = state.game.deref().borrow_mut();
            let moves = game_mut.gen_moves();
            let i = rand::random_range(0..moves.len());
            let mv = &moves[i];
            let move_res = game_mut.make_move(mv);
            match move_res {
                Ok(_) =>  Ok(Signal::Message(String::new())),
                Err(err) =>  Ok(Signal::Message(err.to_string()))
            }

        }
        _ => Ok(Signal::Continue),
    }
}

fn handle_position(cmd: Vec<&str>) -> Result<Signal, Signal> {
    let fen = cmd[1..]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    let pos = match fen.as_str() {
        "startpos" => Game::default(),
        "empty" => Game::empty(),
        _ => match Game::from_fen(&fen) {
            Ok(game) => {
                game
            },
            Err(err) => return Err(Signal::Message(err.to_string()))
        }
    };

    Ok(Signal::Game(Box::from(pos)))
}
