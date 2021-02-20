use spectral::prelude::*;

use functional_testing::http::{Client, Method, StatusCode};
use functional_testing::response::CreatedRoomResponse;

use crate::helpers::GAME_SERVER_HOST;

#[tokio::test]
async fn creates_room_with_room_id() {
    let response = Client::new()
        .request(
            Method::POST,
            format!("http://{}/game/rooms", GAME_SERVER_HOST.to_string()).as_str(),
        )
        .send()
        .await
        .unwrap();

    assert_that(&response.status()).is_equal_to(StatusCode::CREATED);

    asserting("response contains a valid Uuid Id")
        .that(&response.json::<CreatedRoomResponse>().await)
        .is_ok();
}
