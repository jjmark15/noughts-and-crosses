use lazy_static::lazy_static;

use functional_testing::response::RegisteredUserResponse;
use functional_testing::AppClient;
use uuid::Uuid;

lazy_static! {
    static ref GAME_SERVER_HOST: String = {
        match std::env::var("GAME_SERVER_HOST") {
            Ok(val) => val,
            Err(_) => "127.0.0.1:3030".to_string(),
        }
    };
}

pub fn app_client() -> AppClient {
    AppClient::new(GAME_SERVER_HOST.clone())
}

pub async fn create_user(app_client: &AppClient) -> Uuid {
    app_client
        .register_user("name")
        .await
        .json::<RegisteredUserResponse>()
        .await
        .unwrap()
        .id()
}
