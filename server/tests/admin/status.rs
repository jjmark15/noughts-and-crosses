use spectral::prelude::*;

use functional_testing::http::{Client, Method, StatusCode};
use functional_testing::response::AppStatusResponse;

use crate::common::GAME_SERVER_HOST;

#[tokio::test]
async fn returns_status() {
    let response = Client::new()
        .request(
            Method::GET,
            format!("http://{}/admin/status", GAME_SERVER_HOST.to_string()).as_str(),
        )
        .send()
        .await
        .unwrap();

    assert_that(&response.status()).is_equal_to(StatusCode::OK);

    let app_status: AppStatusResponse = response.json().await.unwrap();
    assert_that(&app_status).is_equal_to(&AppStatusResponse::new("OK".to_string()));
}
