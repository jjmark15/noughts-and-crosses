use reqwest::Method;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::http::Response as TungsteniteResponse;
use tokio_tungstenite::tungstenite::Result as TungsteniteResult;
use tokio_tungstenite::{connect_async, WebSocketStream};
use url::{ParseError, Url};
use uuid::Uuid;

use crate::app_client::routes::Route;

mod routes;

pub struct AppClient {
    server_address: String,
    http_client: reqwest::Client,
    socket_connection: Option<WebSocketStream<TcpStream>>,
}

impl AppClient {
    pub fn new(server_address: String) -> Self {
        AppClient {
            server_address,
            http_client: reqwest::Client::new(),
            socket_connection: None,
        }
    }

    fn http_request_base_url(&self, route: Route) -> Result<Url, ParseError> {
        Url::parse(
            format!(
                "http://{}/{}",
                self.server_address.as_str(),
                route.url_path()
            )
            .as_str(),
        )
    }

    fn websockets_connection_url(&self, route: Route) -> Result<Url, ParseError> {
        Url::parse(format!("ws://{}/{}", self.server_address.as_str(), route.url_path()).as_str())
    }

    async fn build_and_send_request(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> reqwest::Response {
        request_builder.send().await.unwrap()
    }

    pub async fn status(&self) -> reqwest::Response {
        let request = self.http_client.request(
            Method::GET,
            self.http_request_base_url(Route::Status).unwrap(),
        );

        self.build_and_send_request(request).await
    }

    pub async fn create_room(&self, user_id: Uuid) -> reqwest::Response {
        let request = self
            .http_client
            .request(
                Method::POST,
                self.http_request_base_url(Route::CreateRoom).unwrap(),
            )
            .header("user-id", user_id.to_string());

        self.build_and_send_request(request).await
    }

    pub async fn join_room(
        &mut self,
        user_id: Uuid,
        room_id: Uuid,
    ) -> TungsteniteResult<TungsteniteResponse<()>> {
        let connection_url = self
            .websockets_connection_url(Route::JoinRoom(room_id))
            .unwrap();

        let (socket, response) = connect_async(
            tokio_tungstenite::tungstenite::http::Request::builder()
                .uri(connection_url.as_str())
                .header("user-id", user_id.to_string())
                .body(())
                .unwrap(),
        )
        .await?;
        self.socket_connection = Some(socket);
        Ok(response)
    }

    pub async fn register_user(&self, name: impl ToString) -> reqwest::Response {
        let request = self.http_client.request(
            Method::POST,
            self.http_request_base_url(Route::RegisterUser(name.to_string()))
                .unwrap(),
        );

        self.build_and_send_request(request).await
    }

    pub async fn user_name(&self, user_id: impl ToString) -> reqwest::Response {
        let request = self.http_client.request(
            Method::GET,
            self.http_request_base_url(Route::UserName(user_id.to_string()))
                .unwrap(),
        );

        self.build_and_send_request(request).await
    }

    pub async fn close_socket_connection(&mut self) {
        if let Some(socket) = &mut self.socket_connection {
            socket.close(None).await.unwrap();
        }
        self.socket_connection = None;
    }
}
