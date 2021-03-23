use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use uuid::Uuid;

use crate::domain::user::{User, UserNotFoundError, UserPersistenceError, UserRepository};

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
        let mut map = self.inner.lock();
        map.insert(user.id(), user.into());
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), UserPersistenceError> {
        self.get(user.id()).await?;
        self.store(user).await
    }

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError> {
        let map = self.inner.lock();
        let stored_user = map
            .get(&id)
            .ok_or_else(|| UserPersistenceError::NotFound(UserNotFoundError::new(id)))?;
        let user = User::new(id, stored_user.name.to_string());
        Ok(user)
    }
}

struct StoredUser {
    name: String,
}

impl StoredUser {
    fn new(name: String) -> Self {
        StoredUser { name }
    }
}

impl From<&User> for StoredUser {
    fn from(user: &User) -> Self {
        StoredUser::new(user.name().to_string())
    }
}
