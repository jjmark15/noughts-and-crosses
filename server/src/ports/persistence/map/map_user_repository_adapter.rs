use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::user::{User, UserPersistenceError, UserRepository};

type EmbeddedDb = Arc<Mutex<HashMap<Uuid, String>>>;

pub(crate) struct MapUserRepositoryAdapter {
    inner: EmbeddedDb,
}

impl MapUserRepositoryAdapter {
    pub(crate) fn new() -> Self {
        MapUserRepositoryAdapter {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for MapUserRepositoryAdapter {
    async fn store(&self, user: &User) -> Result<(), UserPersistenceError> {
        let mut map = self.inner.lock().await;
        map.insert(user.id(), user.name().to_string());
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError> {
        let map = self.inner.lock().await;
        let user_name = map.get(&id).ok_or(UserPersistenceError::UserNotFound(id))?;
        Ok(User::new(id, user_name.to_string()))
    }
}
