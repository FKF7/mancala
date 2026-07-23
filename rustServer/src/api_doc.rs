use utoipa::OpenApi;

use crate::mancala_controller::SimulationResponse;
use crate::mancala_game_data_handler::Hint;
use crate::mancala_game_model::MancalaGame;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Mancala API",
        version = "1.0.0",
        description = "REST API for Mancala game operations, simulation and state analysis."
    ),
    paths(
        crate::mancala_controller::handle_make_move_request,
        crate::mancala_controller::handle_get_hint_data_request,
        crate::mancala_controller::handle_simulate_request,
        crate::mancala_controller::handle_print_path_request,
        crate::mancala_controller::handle_print_sequence_request,
        crate::mancala_controller::handle_decode_code_request
    ),
    components(
        schemas(
            MancalaGame,
            Hint,
            SimulationResponse
        )
    ),
    tags(
        (
            name = "Game",
            description = "Game-related operations"
        ),
        (
            name = "Simulation",
            description = "Simulation endpoints"
        ),
        (
            name = "Debug",
            description = "Debugging utilities"
        )
    )
)]
pub struct ApiDoc;