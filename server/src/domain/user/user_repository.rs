use uuid::Uuid;

use crate::domain::user::{User, UserNotFoundError};

#[async_trait::async_trait]
pub(crate) trait UserRepository {
    async fn store(&self, user: &User) -> Result<(), UserPersistenceError>;

    async fn update(&self, user: &User) -> Result<(), UserPersistenceError>;

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum UserPersistenceError {
    #[error(transparent)]
    NotFound(#[from] UserNotFoundError),
}
