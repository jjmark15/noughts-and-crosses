use spectral::prelude::*;
use uuid::Uuid;
use warp::http::StatusCode;

use functional_testing::TungsteniteError;

use crate::helpers::{app_client, create_user};

#[tokio::test]
async fn creates_room_with_room_id() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;

    let response = app_client.join_new_room(user_id).await.unwrap();
    let room_id_string = response.headers().get("room-id").unwrap().to_str().unwrap();

    assert_that(&Uuid::parse_str(room_id_string)).is_ok();
    app_client.close_socket_connection().await;
}

#[tokio::test]
async fn fails_to_join_different_room_if_already_in_a_room() {
    let mut first_app_client = app_client();
    let mut second_app_client = app_client();
    let user_id = create_user(&first_app_client).await;

    first_app_client.join_new_room(user_id).await.unwrap();

    let second_join_result = second_app_client.join_new_room(user_id).await;

    if let TungsteniteError::Http(response) = second_join_result.err().unwrap() {
        assert_that(&response.status()).is_equal_to(&StatusCode::CONFLICT);
    } else {
        panic!("Unexpected error variant");
    }

    first_app_client.close_socket_connection().await;
    second_app_client.close_socket_connection().await;
}
