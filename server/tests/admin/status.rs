use spectral::prelude::*;

use functional_testing::http::{Client, Method, StatusCode};

use crate::common::server_handle;

#[tokio::test]
async fn returns_status() {
    let _server_handle = server_handle();

    let resp = Client::new()
        .request(Method::GET, "http://localhost:3030/admin/status")
        .send()
        .await
        .unwrap();

    assert_that(&resp.status()).is_equal_to(StatusCode::OK);

    let status_response: StatusResponse = resp.json().await.unwrap();
    assert_that(&status_response).is_equal_to(&StatusResponse::new("OK".to_string()));
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
struct StatusResponse {
    status: String,
}

impl StatusResponse {
    fn new(status: String) -> Self {
        StatusResponse { status }
    }
}
