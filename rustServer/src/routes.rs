use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_doc::ApiDoc;
use crate::mancala_controller::{handle_make_move_request, handle_get_hint_data_request, handle_simulate_request, handle_print_path_request};

pub fn build_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/mancala/make_move", get(handle_make_move_request))
        .route("/api/mancala/get_hint_data", get(handle_get_hint_data_request))
        .route("/api/mancala/simulate", get(handle_simulate_request))
        .route("/api/mancala/print_path", get(handle_print_path_request))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
}
