use crate::mancala_game_model::{MancalaGame};
use crate::types::Pit;
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use axum::{extract::Query, Json};
// use crate::mancala_game_data_handler::MancalaStateFile;
// use crate Constants;
use crate::mancala_game_data_handler::{DataHandler, Hint, HintData};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MoveQuery {
    /// Chosen pit for the move (1-6)
    pit: Pit,

    /// MancalaGame in json format
    game: String
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct BoardQuery {
    /// MancalaGame in json format
    game: String
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SimulationQuery {
    /// MancalaGame in json format
    game: String,


    /// Optional maximum number of ends to simulate
    /// If zero, uses default value
    /// If None, simulates with no limit
    ends: Option<u32>
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct SimulationResponse {
    /// Hints available for the six pits
    #[schema(value_type = Vec<Hint>)]
    hints: HintData,

    /// Number of ends simulated
    ends: u32,
}

#[utoipa::path(
    get,
    path = "/api/mancala/make_move",
    params(MoveQuery),
    responses(
        (
            status = 200,
            description = "Move executed successfully",
            body = MancalaGame
        ),
        (
            status = 400,
            description = "Invalid game state or illegal move"
        )
    ),
    tag = "Game"
)]
pub async fn handle_make_move_request(Query(q): Query<MoveQuery>) -> Json<MancalaGame> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let pit: Pit = q.pit;

    let new_game = game.make_move(pit).expect("Invalid Move");
    DataHandler::save_data(new_game, game, pit).unwrap(); // Handle the error appropriately in a real application

    Json(new_game)
}

#[utoipa::path(
    get,
    path = "/api/mancala/get_hint_data",
    params(BoardQuery),
    responses(
        (
            status = 200,
            description = "Available hints for each playable pit",
            body = Vec<Hint>
        ),
        (
            status = 400,
            description = "Invalid game state"
        ),
        (
            status = 500,
            description = "Unable to retrieve hint data"
        )
    ),
    tag = "Game"
)]
pub async fn handle_get_hint_data_request(Query(q): Query<BoardQuery>) -> Json<HintData> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");

    let hint_data = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    Json(hint_data)
}

#[utoipa::path(
    get,
    path = "/api/mancala/simulate",
    params(SimulationQuery),
    responses(
        (
            status = 200,
            description = "Simulation completed successfully",
            body = SimulationResponse
        ),
        (
            status = 400,
            description = "Invalid simulation request"
        ),
        (
            status = 500,
            description = "Simulation failed"
        )
    ),
    tag = "Simulation"
)]
pub async fn handle_simulate_request(Query(q): Query<SimulationQuery>) -> Json<SimulationResponse> {
    // simulate all or maybe add a limit possible outcomes from a certain point, and return the updated hints obtained after that
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let max_ends: Option<u32> = q.ends;
    
    let ends = DataHandler::simulate_until_moves(game, 0, max_ends).expect("Something went wrong while simulating");
    let hints = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    println!("Simulated until {} ends", ends);
    Json(SimulationResponse { hints, ends })
}

#[utoipa::path(
    get,
    path = "/api/mancala/print_path",
    params(BoardQuery),
    responses(
        (
            status = 200,
            description = "State path printed successfully"
        ),
        (
            status = 400,
            description = "Invalid game state"
        ),
        (
            status = 500,
            description = "Unable to determine state path"
        )
    ),
    tag = "Debug"
)]
pub async fn handle_print_path_request(Query(q): Query<BoardQuery>) -> Json<()> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");

    let path = DataHandler::fetch_path(game).expect("Failed to retrieve path");
    println!("{:?}", path);

    Json(())
}