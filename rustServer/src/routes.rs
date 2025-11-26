use axum::{routing::post, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::mancala_controller::{handle_make_move_request};

pub fn build_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/mancala/make_move", get(handle_make_move_request))
        // .route("/api/mancala/reset", post(mancala_controller::reset_game))
        .layer(cors)
}
