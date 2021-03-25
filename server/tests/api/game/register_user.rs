use spectral::prelude::*;

use nc_test_client::http::StatusCode;
use nc_test_client::response::RegisteredUserResponse;

use crate::helpers::new_app_client;

#[tokio::test]
async fn registers_user_with_name() {
    let app_client = new_app_client();

    let register_response = app_client.register_user("Name").await;

    assert_that(&register_response.status()).is_equal_to(StatusCode::CREATED);

    let id = register_response
        .json::<RegisteredUserResponse>()
        .await
        .unwrap()
        .user_id()
        .unwrap();
    let name = app_client.user_name(id).await.text().await.unwrap();

    assert_that(&name).is_equal_to(&"Name".to_string());
}

#[tokio::test]
async fn registers_user_with_name_including_spaces() {
    let app_client = new_app_client();

    let register_response = app_client.register_user("First Last").await;

    assert_that(&register_response.status()).is_equal_to(StatusCode::CREATED);

    let id = register_response
        .json::<RegisteredUserResponse>()
        .await
        .unwrap()
        .user_id()
        .unwrap();
    let name = app_client.user_name(id).await.text().await.unwrap();

    assert_that(&name).is_equal_to(&"First Last".to_string());
}
