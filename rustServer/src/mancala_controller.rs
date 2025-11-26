use crate::mancala_game_model::{MancalaGame, Pit};
use serde::Deserialize;
use axum::{extract::Query, Json};
// use crate Constants;

#[derive(Deserialize)]
pub struct MakeMoveQuery {
    pit: Pit,
    game: String
}

pub async fn handle_make_move_request(Query(q): Query<MakeMoveQuery>) -> Json<MancalaGame> {
    let game: MancalaGame = serde_json::from_str(&q.game).expect("invalid game JSON");
    let pit = q.pit;
    return Json(game.make_move(pit)); 
}
