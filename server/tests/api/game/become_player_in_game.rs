use spectral::prelude::*;

use nc_test_client::http::StatusCode;
use nc_test_client::response::SimpleErrorResponse;

use crate::helpers::{
    app_client, become_player, create_room, create_user, join_room, non_existent_id, start_new_game,
};

#[tokio::test]
async fn user_becomes_player_in_game() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;

    let become_player_response = app_client.become_player(user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::ACCEPTED);
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_does_nothing_if_user_is_already_player() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;

    app_client.become_player(user_id, room_id).await;

    let become_player_response = app_client.become_player(user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_MODIFIED);
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_fails_if_user_is_not_in_room() {
    let mut app_client = app_client();
    let not_in_room_user_id = create_user(&app_client).await;
    let in_room_user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, in_room_user_id).await;
    join_room(&mut app_client, in_room_user_id, room_id).await;
    start_new_game(&app_client, in_room_user_id, room_id).await;

    let become_player_response = app_client.become_player(not_in_room_user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = become_player_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&format!(
        "User({}) is not a member of Room({})",
        not_in_room_user_id, room_id
    ));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_fails_if_user_does_not_exist() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let fake_user_id = non_existent_id();
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;

    let become_player_response = app_client.become_player(fake_user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = become_player_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&format!("Could not find user with id: {}", fake_user_id));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_fails_if_room_does_not_exist() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = non_existent_id();

    let become_player_response = app_client.become_player(user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = become_player_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&format!("Could not find room with id: {}", room_id));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_fails_if_there_is_no_active_game() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;

    let become_player_response = app_client.become_player(user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = become_player_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&format!(
        "There is no currently active game for room with id: {}",
        room_id
    ));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn become_player_fails_if_there_are_already_two_players() {
    let mut app_client_1 = app_client();
    let mut app_client_2 = app_client();
    let mut app_client_3 = app_client();
    let user_id_1 = create_user(&app_client_1).await;
    let user_id_2 = create_user(&app_client_2).await;
    let user_id_3 = create_user(&app_client_3).await;
    let room_id = create_room(&app_client_1, user_id_1).await;
    join_room(&mut app_client_1, user_id_1, room_id).await;
    join_room(&mut app_client_2, user_id_2, room_id).await;
    join_room(&mut app_client_3, user_id_3, room_id).await;
    start_new_game(&app_client_1, user_id_1, room_id).await;
    become_player(&mut app_client_1, user_id_1, room_id).await;
    become_player(&mut app_client_2, user_id_2, room_id).await;

    let become_player_response = app_client_3.become_player(user_id_3, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = become_player_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&"Exceeded player count limit".to_string());
    app_client_1.close_socket_connection().await;
    app_client_2.close_socket_connection().await;
    app_client_3.close_socket_connection().await;
}

#[tokio::test]
async fn player_is_removed_if_leaves_room() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&mut app_client, user_id, room_id).await;

    app_client.close_socket_connection().await;
    join_room(&mut app_client, user_id, room_id).await;
    let become_player_response = app_client.become_player(user_id, room_id).await;

    assert_that(&become_player_response.status()).is_equal_to(&StatusCode::ACCEPTED);
    app_client.close_socket_connection().await;
}
