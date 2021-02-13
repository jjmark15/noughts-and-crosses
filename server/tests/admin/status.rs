use spectral::prelude::*;

use functional_testing::http::{Client, Method, StatusCode};
use functional_testing::response::AppStatusResponse;

use crate::common::server_handle;

#[tokio::test]
async fn returns_status() {
    let _server_handle = server_handle();

    let response = Client::new()
        .request(Method::GET, "http://localhost:3030/admin/status")
        .send()
        .await
        .unwrap();

    assert_that(&response.status()).is_equal_to(StatusCode::OK);

    let app_status: AppStatusResponse = response.json().await.unwrap();
    assert_that(&app_status).is_equal_to(&AppStatusResponse::new("OK".to_string()));
}
