use std::sync::Arc;

use tokio::sync::Mutex;

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
}

struct StoredUser {
    _id: uuid::Uuid,
    _name: String,
}

impl StoredUser {
    fn from_user(user: &User) -> Self {
        StoredUser {
            _id: user.id(),
            _name: user.name().to_string(),
        }
    }
}
