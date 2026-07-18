use crate::mancala_game_model::{MancalaGame};
use crate::types::Pit;
use serde::{Serialize, Deserialize};
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

#[derive(Deserialize)]
pub struct SimulationQuery {
    game: String,
    ends: Option<u32>
}

#[derive(Serialize, Debug, Clone)]
pub struct SimulationResponse {
    hints: HintData,
    ends: u32,
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

pub async fn handle_simulate_request(Query(q): Query<SimulationQuery>) -> Json<SimulationResponse> {
    // simulate all or maybe add a limit possible outcomes from a certain point, and return the updated hints obtained after that
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let max_ends: Option<u32> = q.ends;
    
    let ends = DataHandler::simulate_until_moves(game, 0, max_ends).expect("Something went wrong while simulating");
    let hints = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    println!("Simulated until {} ends", ends);
    Json(SimulationResponse { hints, ends })
}

pub async fn handle_print_path_request(Query(q): Query<BoardQuery>) -> Json<()> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");

    let path = DataHandler::fetch_path(game).expect("Failed to retrieve path");
    println!("{:?}", path);

    Json(())
}