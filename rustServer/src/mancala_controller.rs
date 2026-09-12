use std::print;

use crate::mancala_game_model::{MancalaGame};
use crate::types::{EncodedGameState, MancalaTurn, Pebbles, Pit};
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use axum::{extract::Query, Json};
// use crate::mancala_game_data_handler::MancalaStateFile;
use crate::mancala_game_data_handler::{DataHandler, Hint, HintData};
use crate::mancala_game_codec::MancalaGameCodec;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MoveQuery {
    /// Chosen pit for the move (1-6)
    pit: Pit,

    /// MancalaGame in json format
    game: MancalaGame
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct BoardQuery {
    /// MancalaGame in json format
    game: MancalaGame
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SimulationQuery {
    /// MancalaGame in json format
    game: MancalaGame,

    /// Optional maximum number of ends to simulate
    /// If zero, uses default value
    /// If None, simulates with no limit
    ends: Option<u32>
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PrintSequenceQuery {
    /// string representing a 64-bit hex code of the game state
    code: String,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DecodeQuery {
    /// string representing a 64-bit hex code of the game state
    code: String,

    /// Current player's turn (0 or 1)
    current_turn: MancalaTurn,

    /// Number of pebbles in player 1's mancala
    current_mancala: Pebbles,

    /// Number of pebbles in player 2's mancala
    opponent_mancala: Pebbles,
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
    post,
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
pub async fn handle_make_move_request(Json(payload): Json<MoveQuery>) -> Json<MancalaGame> {
    let game = payload.game;
    // let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let pit: Pit = payload.pit;

    let new_game = game.make_move(pit).expect("Invalid Move");
    DataHandler::new().save_data(new_game, game, pit).unwrap(); // Handle the error appropriately in a real application

    Json(new_game)
}

#[utoipa::path(
    post,
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
pub async fn handle_get_hint_data_request(Json(payload): Json<BoardQuery>) -> Json<HintData> {
    let game = payload.game;

    let hint_data = DataHandler::fetch_hint_data(game).expect("Failed to retrieve hints");

    Json(hint_data)
}

#[utoipa::path(
    post,
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
pub async fn handle_simulate_request(Json(payload): Json<SimulationQuery>) -> Json<SimulationResponse> {
    // simulate all or maybe add a limit possible outcomes from a certain point, and return the updated hints obtained after that
    let game = payload.game;
    let max_ends: Option<u32> = payload.ends;
    
    let total_simulate_time = std::time::Instant::now();
    let mut data_handler = DataHandler::new();
    let ends = data_handler.simulate_until_moves(game, 0, max_ends).expect("Something went wrong while simulating");
    println!("Total simulation time: {:?}", total_simulate_time.elapsed());
    data_handler.print_timings();
    
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
pub async fn handle_print_path_request(Query(payload): Query<BoardQuery>) -> Json<()> {
    let game = payload.game;
    // let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");

    let path = DataHandler::fetch_path(game).expect("Failed to retrieve path");
    println!("{:?}", path);

    Json(())
}

#[utoipa::path(
    get,
    path = "/api/mancala/print_sequence",
    params(PrintSequenceQuery),
    responses(
        (
            status = 200,
            description = "Sequence printed successfully"
        ),
        (
            status = 400,
            description = "Invalid game state"
        ),
        (
            status = 500,
            description = "Unable to determine sequence"
        )
    ),
    tag = "Debug"
)]
pub async fn handle_print_sequence_request(Json(payload): Json<PrintSequenceQuery>) -> Json<()> {
    let code: EncodedGameState = u64::from_str_radix(&payload.code, 16).expect("Invalid hex code");

    let sequence = DataHandler::fetch_sequence(code).expect("Failed to retrieve sequence");
    println!("{}", sequence);

    Json(())
}

#[utoipa::path(
    get,
    path = "/api/mancala/decode_code",
    params(DecodeQuery),
    responses(
        (
            status = 200,
            description = "Code decoded successfully",
            body = MancalaGame
        ),
        (
            status = 400,
            description = "Invalid code or game state"
        )
    ),
    tag = "Debug"
)]
pub async fn handle_decode_code_request(Query(q): Query<DecodeQuery>) -> Json<MancalaGame> {
    let code: EncodedGameState = u64::from_str_radix(&q.code, 16).expect("Invalid hex code");
    let current_turn = q.current_turn;
    let current_mancala = q.current_mancala;
    let opponent_mancala = q.opponent_mancala;

    let game = MancalaGameCodec::decode(code, current_turn, current_mancala, opponent_mancala).expect("Failed to decode game");
    Json(game)
}