use lazy_static::lazy_static;

use functional_testing::response::{CreateRoomResponse, RegisteredUserResponse};
use functional_testing::AppClient;
use uuid::Uuid;

lazy_static! {
    static ref GAME_SERVER_HOST: String = {
        match std::env::var("GAME_SERVER_HOST") {
            Ok(val) => val,
            Err(_) => "localhost:3030".to_string(),
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
        .user_id()
        .unwrap()
}

pub async fn create_room(app_client: &AppClient, user_id: Uuid) -> Uuid {
    app_client
        .create_room(user_id)
        .await
        .json::<CreateRoomResponse>()
        .await
        .unwrap()
        .room_id()
        .unwrap()
}
