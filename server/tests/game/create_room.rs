use spectral::prelude::*;

use functional_testing::http::{Client, Method, StatusCode};

use crate::common::server_handle;
use functional_testing::response::CreatedRoomResponse;

#[tokio::test]
async fn creates_room_with_room_id() {
    let _server_handle = server_handle();

    let response = Client::new()
        .request(Method::POST, "http://localhost:3030/game/room")
        .send()
        .await
        .unwrap();

    assert_that(&response.status()).is_equal_to(StatusCode::CREATED);

    asserting("response contains a valid Uuid Id")
        .that(&response.json::<CreatedRoomResponse>().await)
        .is_ok();
}
