use crate::game::structs::game::Game;

use std::{cell::Ref, ops::Deref} ;

use super::state::{Signal, State};

pub fn handle_move(state: &mut State, cmd: Vec<&str>) -> Result<Signal, ()> {
    let game = state.game.borrow();
    let mv = match game.parse_move(cmd[1]) {
        Err(_) => {
            return Ok(Signal::Message(String::from("Incorrect move!")))
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

pub fn handle_random(state: &mut State) -> Result<Signal, ()> {
    let mut game_mut = state.game.deref().borrow_mut();
    let moves = game_mut.gen_moves();
    if moves.is_empty() {
        Ok(Signal::Message("No moves available".to_string()))
    }else{
        let i = rand::random_range(0..moves.len());
        let mv = &moves[i];
        let move_res = game_mut.make_move(mv);
        match move_res {
            Ok(_) =>  Ok(Signal::Message(String::new())),
            Err(err) =>  Ok(Signal::Message(err.to_string()))
        }
    }
}

pub fn handle_moves(game: Ref<Game>, cmd: Vec<&str>) -> Result<Signal, Signal> {
    let algebraic = matches!(cmd[1], "a");
    let msg = game.gen_moves()
        .iter()
        .fold(String::from("Available moves: "), |acc, m| {
            acc + format!("{} ", if algebraic { m.algebraic() } else { m.to_string() }).as_str()
        });
    Ok(Signal::Message(msg))
    
}

pub fn handle_position(cmd: Vec<&str>) -> Result<Signal, Signal> {
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
