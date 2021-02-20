use lazy_static::lazy_static;

use functional_testing::app_client::AppClient;

lazy_static! {
    static ref APP_CLIENT: AppClient = {
        let server_host = match std::env::var("GAME_SERVER_HOST") {
            Ok(val) => val,
            Err(_) => "localhost:3030".to_string(),
        };

        AppClient::new(server_host)
    };
}

pub fn app_client() -> &'static AppClient {
    &APP_CLIENT
}
