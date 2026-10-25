use axum::{routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_doc::ApiDoc;
use crate::mancala_controller::{
    handle_make_move_request,
    handle_get_hint_data_request,
    handle_simulate_request,
    handle_print_path_request,
    handle_print_sequence_request,
    handle_decode_code_request
};

pub fn build_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/mancala/make_move", post(handle_make_move_request))
        .route("/api/mancala/get_hint_data", post(handle_get_hint_data_request))
        .route("/api/mancala/simulate", post(handle_simulate_request))
        .route("/api/mancala/print_path", post(handle_print_path_request))
        .route("/api/mancala/print_sequence", post(handle_print_sequence_request))
        .route("/api/mancala/decode_code", post(handle_decode_code_request))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
}
