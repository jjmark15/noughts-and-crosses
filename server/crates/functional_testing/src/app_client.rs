use std::fmt::Display;

use reqwest::Method;
use uuid::Uuid;

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

    pub async fn register_user<S: AsRef<str> + Display>(&self, name: S) -> reqwest::Response {
        let request = self.http_client.request(
            Method::POST,
            self.base_request_url(format!("/game/users/{}", name).as_str())
                .as_str(),
        );

        self.build_and_send_request(request).await
    }

    pub async fn user_name(&self, user_id: impl Into<UuidOrString>) -> reqwest::Response {
        let request = self.http_client.request(
            Method::GET,
            self.base_request_url(format!("/game/users/{}", user_id.into().id_string()).as_str())
                .as_str(),
        );

        self.build_and_send_request(request).await
    }
}

pub enum UuidOrString {
    Uuid(Uuid),
    String(String),
}

impl UuidOrString {
    fn id_string(&self) -> String {
        match self {
            UuidOrString::String(s) => s.to_string(),
            UuidOrString::Uuid(uuid) => uuid.to_string(),
        }
    }
}

impl From<String> for UuidOrString {
    fn from(s: String) -> Self {
        UuidOrString::String(s)
    }
}

impl From<Uuid> for UuidOrString {
    fn from(id: Uuid) -> Self {
        UuidOrString::Uuid(id)
    }
}
