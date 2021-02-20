use uuid::Uuid;

use crate::domain::user::User;

#[async_trait::async_trait]
pub(crate) trait UserRepository {
    async fn store(&self, user: &User) -> Result<(), UserPersistenceError>;

    async fn get(&self, id: Uuid) -> Result<User, UserPersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum UserPersistenceError {
    #[error("Could not find user with id: {0}")]
    UserNotFound(Uuid),
}
