use crate::mancala_game_model::{MancalaGame};
use crate::types::Pit;
use serde::Deserialize;
use axum::{extract::Query, Json};
// use crate::mancala_game_data_handler::MancalaStateFile;
// use crate Constants;
use crate::mancala_game_data_handler::{DataHandler, HintData};

#[derive(Deserialize)]
pub struct MoveQuery {
    pit: Pit,
    game: String
}

#[derive(Deserialize)]
pub struct BoardQuery {
    game: String
}

pub async fn handle_make_move_request(Query(q): Query<MoveQuery>) -> Json<MancalaGame> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let pit: Pit = q.pit;

    let new_game = game.make_move(pit).expect("Invalid Move");
    DataHandler::save_data(new_game, game, pit).unwrap(); // Handle the error appropriately in a real application

    Json(new_game)
}

pub async fn handle_get_hint_data_request(Query(q): Query<BoardQuery>) -> Json<HintData> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");

    let hint_data = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    Json(hint_data)
}

pub async fn handle_simulate_request(Query(q): Query<BoardQuery>) -> Json<HintData> {
    // simulate all or maybe add a limit possible outcomes from a certain point, and return the updated hints obtained after that
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    
    DataHandler::simulate_until_moves(game, 0).expect("Something went wrong while simulating");
    let hint_data = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    Json(hint_data)
}