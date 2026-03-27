mod mancala_controller;
mod routes;
mod error;
mod mancala_game_codec;
mod constants;
mod types;
mod mancala_game_model;
mod mancala_game_data_handler;

use std::net::SocketAddr;
use crate::mancala_game_data_handler::{MancalaStateFile};
use crate::constants::DEFAULT_BOARD_CODE;


#[tokio::main]
async fn main() {
    MancalaStateFile::new_empty(DEFAULT_BOARD_CODE, None).save_as_new().unwrap(); // Handle the error appropriately in a real application

    let app = routes::build_router();

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Servidor em http://{address}");
    axum::serve(tokio::net::TcpListener::bind(address).await.unwrap(), app)
        .await
        .unwrap();
}
