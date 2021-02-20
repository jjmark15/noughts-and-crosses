use reqwest::Method;

pub struct AppClient {
    server_address: String,
    http_client: reqwest::Client,
}

impl AppClient {
    pub fn new(server_address: String) -> Self {
        AppClient {
            server_address,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn status(&self) -> reqwest::Response {
        self.http_client
            .request(
                Method::GET,
                format!("http://{}/admin/status", self.server_address.as_str()).as_str(),
            )
            .send()
            .await
            .unwrap()
    }

    pub async fn create_room(&self) -> reqwest::Response {
        self.http_client
            .request(
                Method::POST,
                format!("http://{}/game/rooms", self.server_address.as_str()).as_str(),
            )
            .send()
            .await
            .unwrap()
    }
}
