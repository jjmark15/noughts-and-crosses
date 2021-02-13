use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::room::{Room, RoomPersistenceError, RoomRepository};

type EmbeddedDb = Arc<Mutex<Vec<Uuid>>>;

pub(crate) struct VecRoomRepositoryAdapter {
    inner: EmbeddedDb,
}

impl VecRoomRepositoryAdapter {
    pub(crate) fn new() -> Self {
        VecRoomRepositoryAdapter {
            inner: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait::async_trait]
impl RoomRepository for VecRoomRepositoryAdapter {
    async fn store(&self, room: &Room) -> Result<(), RoomPersistenceError> {
        let mut vec = self.inner.lock().await;
        vec.push(*room.id());
        Ok(())
    }
}
