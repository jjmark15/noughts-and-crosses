use spectral::prelude::*;

use nc_test_client::http::StatusCode;

use crate::helpers::app_client;

#[tokio::test]
async fn returns_status() {
    let app_client = app_client();

    let response = app_client.status().await;

    assert_that(&response.status()).is_equal_to(StatusCode::OK);

    let app_status: String = response.text().await.unwrap();
    assert_that(&app_status).is_equal_to(&"OK".to_string());
}
