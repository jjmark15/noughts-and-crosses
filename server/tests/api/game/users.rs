use spectral::prelude::*;

use functional_testing::http::StatusCode;

use crate::helpers::app_client;
use functional_testing::response::RegisteredUserResponse;

#[tokio::test]
async fn registers_user_with_name() {
    let response = app_client().register_user("Name").await;

    assert_that(&response.status()).is_equal_to(StatusCode::CREATED);

    asserting("response contains a valid Uuid Id")
        .that(&response.json::<RegisteredUserResponse>().await)
        .is_ok();
}
