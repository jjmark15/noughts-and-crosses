use lazy_static::lazy_static;
use spectral::prelude::*;
use uuid::Uuid;
use warp::http::StatusCode;

use functional_testing::response::{CreateRoomResponse, RegisteredUserResponse};
use functional_testing::{AppClient, GameMove};

pub(crate) mod game_moves;

lazy_static! {
    static ref GAME_SERVER_HOST: String = {
        match std::env::var("GAME_SERVER_HOST") {
            Ok(val) => val,
            Err(_) => "localhost:3030".to_string(),
        }
    };
}

pub fn app_client() -> AppClient {
    AppClient::new(GAME_SERVER_HOST.clone())
}

pub fn non_existent_id() -> Uuid {
    Uuid::nil()
}

pub async fn create_user(app_client: &AppClient) -> Uuid {
    app_client
        .register_user("name")
        .await
        .json::<RegisteredUserResponse>()
        .await
        .unwrap()
        .user_id()
        .unwrap()
}

pub async fn create_room(app_client: &AppClient, user_id: Uuid) -> Uuid {
    app_client
        .create_room(user_id)
        .await
        .json::<CreateRoomResponse>()
        .await
        .unwrap()
        .room_id()
        .unwrap()
}

pub async fn join_room(app_client: &mut AppClient, user_id: Uuid, room_id: Uuid) {
    app_client
        .join_room(user_id, room_id)
        .await
        .expect("Failed to join room");
}

pub async fn start_new_game(app_client: &AppClient, user_id: Uuid, room_id: Uuid) {
    let response = app_client.start_new_game(user_id, room_id).await;
    assert_that(&response.status()).is_equal_to(&StatusCode::CREATED);
}

pub async fn become_player(app_client: &AppClient, user_id: Uuid, room_id: Uuid) {
    let response = app_client.become_player(user_id, room_id).await;
    assert_that(&response.status()).is_equal_to(&StatusCode::ACCEPTED);
}

pub async fn make_game_move(
    app_client: &AppClient,
    user_id: Uuid,
    room_id: Uuid,
    game_move: GameMove,
) {
    let response = app_client.make_game_move(user_id, room_id, game_move).await;
    assert_that(&response.status()).is_equal_to(&StatusCode::ACCEPTED);
}
