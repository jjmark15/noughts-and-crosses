use spectral::prelude::*;
use uuid::Uuid;

use functional_testing::http::StatusCode;
use functional_testing::response::SimpleErrorResponse;

use crate::helpers::{app_client, create_room, create_user, join_room};

#[tokio::test]
async fn starts_new_game() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;

    let new_game_response = app_client.start_new_game(user_id, room_id).await;

    assert_that(&new_game_response.status()).is_equal_to(&StatusCode::CREATED);

    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn fails_if_user_is_not_in_room() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;

    let new_game_response = app_client.start_new_game(user_id, room_id).await;

    assert_that(&new_game_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = new_game_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&format!(
        "User({}) is not a member of Room({})",
        user_id, room_id
    ));

    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn fails_if_user_does_not_exist() {
    let mut app_client = app_client();
    let user_id = Uuid::nil();
    let room_id = create_room(&app_client, create_user(&app_client).await).await;

    let new_game_response = app_client.start_new_game(user_id, room_id).await;

    assert_that(&new_game_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = new_game_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(
        &"Could not find user with id: 00000000-0000-0000-0000-000000000000".to_string(),
    );

    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn fails_if_room_does_not_exist() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = Uuid::nil();

    let new_game_response = app_client.start_new_game(user_id, room_id).await;

    assert_that(&new_game_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = new_game_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(
        &"Could not find room with id: 00000000-0000-0000-0000-000000000000".to_string(),
    );

    app_client.close_socket_connection().await;
}
