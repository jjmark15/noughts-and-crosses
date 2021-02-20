use spectral::prelude::*;

use functional_testing::http::StatusCode;
use functional_testing::response::AppStatus;

use crate::helpers::app_client;

#[tokio::test]
async fn returns_status() {
    let response = app_client().status().await;

    assert_that(&response.status()).is_equal_to(StatusCode::OK);

    let app_status: AppStatus = response.json().await.unwrap();
    assert_that(&app_status).is_equal_to(&AppStatus::new("OK".to_string()));
}
