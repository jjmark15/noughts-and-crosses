use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::user::{User, UserPersistenceError, UserRepository};

type EmbeddedDb = Arc<Mutex<HashMap<Uuid, StoredUser>>>;

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
        map.insert(user.id(), user.into());
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), UserPersistenceError> {
        self.get(user.id()).await?;
        self.store(user).await
    }

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError> {
        let map = self.inner.lock().await;
        let stored_user = map.get(&id).ok_or(UserPersistenceError::UserNotFound(id))?;
        let user = User::new(id, stored_user.name.to_string(), stored_user.room_id);
        Ok(user)
    }
}

struct StoredUser {
    name: String,
    room_id: Option<Uuid>,
}

impl StoredUser {
    fn new(name: String, room_id: Option<Uuid>) -> Self {
        StoredUser { name, room_id }
    }
}

impl From<&User> for StoredUser {
    fn from(user: &User) -> Self {
        StoredUser::new(user.name().to_string(), user.room_id())
    }
}
