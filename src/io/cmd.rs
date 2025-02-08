use crate::shared::functions::square_to_index;

use super::{handlers::{handle_move, handle_moves, handle_position, handle_random, handle_unchecked_move}, state::{Signal, State}};

pub fn handle_cmd(state: &mut State, cmd: &Vec<&str>) -> Result<Signal, ()> {
    let mut cmd = cmd.to_vec();
    // Normalizing input length
    while cmd.len() < 5{
        cmd.push("");
    }

    match cmd[0] {
        "q" | "quit" | "exit" => Ok(Signal::Exit),
        "moves" =>  {
            match handle_moves(state.game.borrow(), cmd) {
                Ok(s) => Ok(s),
                Err(s) => Ok(s)
            }
        },
        "i" => {
            Ok(Signal::Message(format!("Index: {}", square_to_index(cmd[1]).unwrap())))
        }
        "m" => handle_move(state, cmd),
        "position" => {
            match handle_position(cmd) {
                Ok(s) => Ok(s),
                Err(s) => Ok(s)
            }
        }
        "r" => handle_random(state),
        _ => Ok(Signal::Continue),
    }
}

pub fn handle_debug_cmd(state: &mut State, cmd: &Vec<&str>) -> Result<Signal, ()> {
    let mut cmd = cmd.to_vec();
    // Normalizing input length
    while cmd.len() < 5{
        cmd.push("");
    }

    match cmd[0] {
        "q" | "quit" | "exit" => Ok(Signal::Exit),
        "moves" =>  {
            match handle_moves(state.game.borrow(), cmd) {
                Ok(s) => Ok(s),
                Err(s) => Ok(s)
            }
        },
        "i" => {
            Ok(Signal::Message(format!("Index: {}", square_to_index(cmd[1]).unwrap())))
        }
        "m" => handle_move(state, cmd),
        "um" => handle_unchecked_move(state, cmd),
        "position" => {
            match handle_position(cmd) {
                Ok(s) => Ok(s),
                Err(s) => Ok(s)
            }
        }
        "r" => handle_random(state),
        // "state" => {
        //     Ok(Signal::Message(state.game.borrow().state.to_string()))
        // }
        "hash" => {
            Ok(Signal::Message(state.game.borrow().get_hash().to_string()))
        }
        "hashes" => {
            Ok(Signal::Message(format!("{:?}", state.game.borrow().repetition_history)))
        }
        _ => Ok(Signal::Continue),
    }
}
