use spectral::prelude::*;
use warp::http::StatusCode;

use functional_testing::TungsteniteError;

use crate::helpers::{app_client, create_room, create_user};

#[tokio::test]
async fn join_room() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;

    assert_that(&app_client.join_room(user_id, room_id).await).is_ok();

    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn fails_to_join_different_room_if_already_in_a_room() {
    let mut first_app_client = app_client();
    let mut second_app_client = app_client();
    let user_id = create_user(&first_app_client).await;
    let room_id = create_room(&first_app_client, user_id).await;
    let other_room_id = create_room(&first_app_client, user_id).await;

    first_app_client.join_room(user_id, room_id).await.unwrap();

    let second_join_result = second_app_client.join_room(user_id, other_room_id).await;

    if let TungsteniteError::Http(response) = second_join_result.err().unwrap() {
        assert_that(&response.status()).is_equal_to(&StatusCode::CONFLICT);
    } else {
        panic!("Unexpected error variant");
    }

    first_app_client.close_socket_connection().await;
    second_app_client.close_socket_connection().await;
}
