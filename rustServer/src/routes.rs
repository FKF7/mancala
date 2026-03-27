use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::mancala_controller::{handle_make_move_request, handle_get_hint_data_request, handle_simulate_request};

pub fn build_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/mancala/make_move", get(handle_make_move_request))
        .route("/api/mancala/get_hint_data", get(handle_get_hint_data_request))
        .route("/api/mancala/simulate", get(handle_simulate_request))
        .layer(cors)
}
