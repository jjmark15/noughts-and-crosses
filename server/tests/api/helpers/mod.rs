use lazy_static::lazy_static;

use functional_testing::AppClient;

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
