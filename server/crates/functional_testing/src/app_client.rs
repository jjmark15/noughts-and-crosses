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

    fn base_request_url(&self, path: &str) -> String {
        format!("http://{}{}", self.server_address.as_str(), path)
    }

    async fn build_and_send_request(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> reqwest::Response {
        request_builder.send().await.unwrap()
    }

    pub async fn status(&self) -> reqwest::Response {
        let request = self
            .http_client
            .request(Method::GET, self.base_request_url("/admin/status").as_str());

        self.build_and_send_request(request).await
    }

    pub async fn create_room(&self) -> reqwest::Response {
        let request = self
            .http_client
            .request(Method::POST, self.base_request_url("/game/rooms").as_str());

        self.build_and_send_request(request).await
    }
}
