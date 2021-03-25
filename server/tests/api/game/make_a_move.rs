use spectral::prelude::*;

use nc_test_client::http::StatusCode;
use nc_test_client::response::SimpleErrorResponse;

use crate::helpers::game_moves::{
    top_left, x_position_above_valid_range, x_position_below_valid_range,
    y_position_above_valid_range, y_position_below_valid_range,
};
use crate::helpers::{
    become_player, create_room, create_user, join_room, make_game_move, new_app_client,
    non_existent_id, start_new_game,
};

#[tokio::test]
async fn user_makes_a_move_in_a_game() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;
    let game_move = top_left();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::ACCEPTED);
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_x_position_is_above_valid_range() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;
    let game_move = x_position_above_valid_range();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&"Position is out of bounds".to_string());
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_x_position_is_below_valid_range() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;
    let game_move = x_position_below_valid_range();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&"Game move request object is invalid".to_string());
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_y_position_is_above_valid_range() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;
    let game_move = y_position_above_valid_range();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&"Position is out of bounds".to_string());
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_y_position_is_below_valid_range() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;
    let game_move = y_position_below_valid_range();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&"Game move request object is invalid".to_string());
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_position_is_already_occupied() {
    let game_move = top_left();

    let mut other_app_client = new_app_client();
    let other_user_id = create_user(&other_app_client).await;
    let room_id = create_room(&other_app_client, other_user_id).await;
    join_room(&mut other_app_client, other_user_id, room_id).await;
    start_new_game(&other_app_client, other_user_id, room_id).await;
    become_player(&other_app_client, other_user_id, room_id).await;
    make_game_move(&other_app_client, other_user_id, room_id, game_move).await;

    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    join_room(&mut app_client, user_id, room_id).await;
    become_player(&app_client, user_id, room_id).await;

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&"Position is already occupied".to_string());
    app_client.close_socket_connection().await;
    other_app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_user_is_not_a_player_in_the_room() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    start_new_game(&app_client, user_id, room_id).await;
    let game_move = top_left();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&format!("User({}) is not a player in game", user_id));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_user_is_not_in_room() {
    let game_move = top_left();

    let mut other_app_client = new_app_client();
    let other_user_id = create_user(&other_app_client).await;
    let room_id = create_room(&other_app_client, other_user_id).await;
    join_room(&mut other_app_client, other_user_id, room_id).await;
    start_new_game(&other_app_client, other_user_id, room_id).await;
    become_player(&other_app_client, other_user_id, room_id).await;

    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_ACCEPTABLE);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&format!(
        "User({}) is not a member of Room({})",
        user_id, room_id
    ));
    app_client.close_socket_connection().await;
    other_app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_user_does_not_exist() {
    let game_move = top_left();

    let mut other_app_client = new_app_client();
    let other_user_id = create_user(&other_app_client).await;
    let room_id = create_room(&other_app_client, other_user_id).await;
    join_room(&mut other_app_client, other_user_id, room_id).await;
    start_new_game(&other_app_client, other_user_id, room_id).await;
    become_player(&other_app_client, other_user_id, room_id).await;

    let mut app_client = new_app_client();
    let user_id = non_existent_id();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&format!("Could not find user with id: {}", user_id));
    app_client.close_socket_connection().await;
    other_app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_there_is_no_active_game_in_room() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    join_room(&mut app_client, user_id, room_id).await;
    let game_move = top_left();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(&format!(
        "There is no currently active game for room with id: {}",
        room_id
    ));
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn move_fails_if_room_does_not_exist() {
    let mut app_client = new_app_client();
    let user_id = create_user(&app_client).await;
    let room_id = non_existent_id();
    let game_move = top_left();

    let game_move_response = app_client.make_game_move(user_id, room_id, game_move).await;

    assert_that(&game_move_response.status()).is_equal_to(&StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = game_move_response.json().await.unwrap();
    assert_that(&error_response.cause())
        .is_equal_to(&format!("Could not find room with id: {}", room_id));
    app_client.close_socket_connection().await;
}
