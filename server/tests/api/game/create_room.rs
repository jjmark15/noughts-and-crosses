use spectral::prelude::*;

use functional_testing::http::StatusCode;
use functional_testing::response::{CreateRoomResponse, SimpleErrorResponse};

use crate::helpers::{app_client, create_user, non_existent_id};

#[tokio::test]
async fn creates_room() {
    let app_client = app_client();
    let user_id = create_user(&app_client).await;

    let create_room_response: CreateRoomResponse =
        app_client.create_room(user_id).await.json().await.unwrap();

    assert_that(&create_room_response.room_id()).is_ok();
}

#[tokio::test]
async fn create_room_fails_given_fake_user_id() {
    let app_client = app_client();
    let user_id = non_existent_id();

    let create_room_response = app_client.create_room(user_id).await;

    assert_that(&create_room_response.status()).is_equal_to(StatusCode::NOT_FOUND);
    let error_response: SimpleErrorResponse = create_room_response.json().await.unwrap();
    assert_that(&error_response.cause()).is_equal_to(
        &"Could not find user with id: 00000000-0000-0000-0000-000000000000".to_string(),
    );
}
