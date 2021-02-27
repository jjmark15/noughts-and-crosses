use spectral::prelude::*;
use uuid::Uuid;

use functional_testing::http::StatusCode;
use functional_testing::response::{RegisteredUserResponse, SimpleErrorResponse};

use crate::helpers::app_client;

#[tokio::test]
async fn registers_user_with_name() {
    let app_client = app_client();

    let register_response = app_client.register_user("Name").await;

    assert_that(&register_response.status()).is_equal_to(StatusCode::CREATED);

    let id = register_response
        .json::<RegisteredUserResponse>()
        .await
        .expect("Response should contain a valid Uuid Id")
        .id();
    let name = app_client.user_name(id).await.text().await.unwrap();

    assert_that(&name).is_equal_to(&"Name".to_string());
}

#[tokio::test]
async fn registers_user_with_name_including_spaces() {
    let app_client = app_client();

    let register_response = app_client.register_user("First Last").await;

    assert_that(&register_response.status()).is_equal_to(StatusCode::CREATED);

    let id = register_response
        .json::<RegisteredUserResponse>()
        .await
        .expect("Response should contain a valid Uuid Id")
        .id();
    let name = app_client.user_name(id).await.text().await.unwrap();

    assert_that(&name).is_equal_to(&"First Last".to_string());
}

#[tokio::test]
async fn returns_name_given_user_id() {
    let app_client = app_client();

    let id = app_client
        .register_user("Name")
        .await
        .json::<RegisteredUserResponse>()
        .await
        .unwrap()
        .id();

    let response = app_client.user_name(id).await;

    assert_that(&response.status()).is_equal_to(StatusCode::FOUND);
    assert_that(&response.text().await.unwrap()).is_equal_to(&"Name".to_string());
}

#[tokio::test]
async fn get_user_name_fails_if_user_with_id_does_not_exist() {
    let app_client = app_client();
    let id = Uuid::nil();

    let response = app_client.user_name(id).await;

    assert_that(&response.status()).is_equal_to(StatusCode::NOT_FOUND);
    assert_that(
        &response
            .json::<SimpleErrorResponse>()
            .await
            .unwrap()
            .cause(),
    )
    .is_equal_to(&"Could not find user with id: 00000000-0000-0000-0000-000000000000".to_string());
}

#[tokio::test]
async fn get_user_name_fails_given_invalid_id() {
    let app_client = app_client();
    let id = "invalid_id";

    let response = app_client.user_name(id.to_string()).await;

    assert_that(&response.status()).is_equal_to(StatusCode::METHOD_NOT_ALLOWED);
}
