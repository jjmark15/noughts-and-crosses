use spectral::prelude::*;

use functional_testing::http::StatusCode;
use functional_testing::response::CreatedRoomResponse;

use crate::helpers::app_client;

#[tokio::test]
async fn creates_room_with_room_id() {
    let response = app_client().create_room().await;

    assert_that(&response.status()).is_equal_to(StatusCode::CREATED);

    asserting("response contains a valid Uuid Id")
        .that(&response.json::<CreatedRoomResponse>().await)
        .is_ok();
}
