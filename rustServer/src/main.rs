mod mancala_controller;
mod routes;
mod error;
pub mod mancala_game_model;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = routes::build_router();

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Servidor em http://{address}");
    axum::serve(tokio::net::TcpListener::bind(address).await.unwrap(), app)
        .await
        .unwrap();
}
