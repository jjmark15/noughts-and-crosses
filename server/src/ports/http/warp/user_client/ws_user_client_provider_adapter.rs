use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::ws::Message;

use crate::domain::user::{UserClientProvider, UserClientProviderError};
use crate::ports::http::warp::WsUserClientAdapter;

type UserClientMap = Arc<Mutex<HashMap<Uuid, Arc<Sender<Result<Message, warp::Error>>>>>>;

pub(crate) struct WsUserClientProviderAdapter {
    inner: UserClientMap,
}

impl WsUserClientProviderAdapter {
    pub(crate) fn new() -> Self {
        WsUserClientProviderAdapter {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) async fn put(&self, user_client: WsUserClientAdapter) {
        let mut map = self.inner.lock().await;
        map.insert(user_client.user_id(), user_client.transmitter());
    }

    pub(crate) async fn remove(&self, user_id: Uuid) {
        let mut map = self.inner.lock().await;
        map.remove(&user_id);
    }
}

#[async_trait::async_trait]
impl UserClientProvider for WsUserClientProviderAdapter {
    type UserClient = WsUserClientAdapter;

    async fn get(&self, user_id: Uuid) -> Result<Self::UserClient, UserClientProviderError> {
        let map = self.inner.lock().await;
        let transmitter = map
            .get(&user_id)
            .ok_or(UserClientProviderError::UserClientNotAvailable)?
            .clone();
        Ok(WsUserClientAdapter::new(user_id, transmitter))
    }
}
