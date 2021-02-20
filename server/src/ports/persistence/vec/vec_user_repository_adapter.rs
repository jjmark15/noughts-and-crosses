use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::user::{User, UserPersistenceError, UserRepository};

type EmbeddedDb = Arc<Mutex<Vec<StoredUser>>>;

pub(crate) struct VecUserRepositoryAdapter {
    inner: EmbeddedDb,
}

impl VecUserRepositoryAdapter {
    pub(crate) fn new() -> Self {
        VecUserRepositoryAdapter {
            inner: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for VecUserRepositoryAdapter {
    async fn store(&self, user: &User) -> Result<(), UserPersistenceError> {
        let mut vec = self.inner.lock().await;
        vec.push(StoredUser::from_user(user));
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError> {
        let vec = self.inner.lock().await;
        let user = vec
            .iter()
            .find(|user| user.id == id)
            .ok_or(UserPersistenceError::UserNotFound(id))?;
        Ok(user.into())
    }
}

struct StoredUser {
    id: uuid::Uuid,
    name: String,
}

impl StoredUser {
    fn from_user(user: &User) -> Self {
        StoredUser {
            id: user.id(),
            name: user.name().to_string(),
        }
    }
}

impl From<&StoredUser> for User {
    fn from(user: &StoredUser) -> Self {
        User::new(user.id, user.name.to_string())
    }
}
