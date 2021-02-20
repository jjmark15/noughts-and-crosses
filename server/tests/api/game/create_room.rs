use spectral::prelude::*;

use functional_testing::http::StatusCode;
use functional_testing::response::CreatedRoomResponse;

use crate::helpers::APP_CLIENT;

#[tokio::test]
async fn creates_room_with_room_id() {
    let response = APP_CLIENT.create_room().await;

    assert_that(&response.status()).is_equal_to(StatusCode::CREATED);

    asserting("response contains a valid Uuid Id")
        .that(&response.json::<CreatedRoomResponse>().await)
        .is_ok();
}