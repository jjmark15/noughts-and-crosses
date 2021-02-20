use spectral::prelude::*;

use functional_testing::http::StatusCode;
use functional_testing::response::AppStatus;

use crate::helpers::APP_CLIENT;

#[tokio::test]
async fn returns_status() {
    let response = APP_CLIENT.status().await;

    assert_that(&response.status()).is_equal_to(StatusCode::OK);

    let app_status: AppStatus = response.json().await.unwrap();
    assert_that(&app_status).is_equal_to(&AppStatus::new("OK".to_string()));
}
