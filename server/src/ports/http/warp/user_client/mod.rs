use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use warp::ws::Message;

pub(crate) use ws_user_client_provider_adapter::*;

use crate::domain::user::UserClient;

mod ws_user_client_provider_adapter;

pub(crate) struct WsUserClientAdapter {
    user_id: Uuid,
    transmitter: Arc<Sender<Result<Message, warp::Error>>>,
}

impl WsUserClientAdapter {
    pub(crate) fn new(
        user_id: Uuid,
        user_ws_tx: Arc<Sender<Result<Message, warp::Error>>>,
    ) -> Self {
        WsUserClientAdapter {
            user_id,
            transmitter: user_ws_tx,
        }
    }

    pub(crate) fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub(crate) fn transmitter(&self) -> Arc<Sender<Result<Message, warp::Error>>> {
        self.transmitter.clone()
    }
}

impl UserClient for WsUserClientAdapter {}
