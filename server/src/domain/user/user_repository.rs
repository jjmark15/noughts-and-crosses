use crate::domain::user::User;

#[async_trait::async_trait]
pub(crate) trait UserRepository {
    async fn store(&self, user: &User) -> Result<(), UserPersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum UserPersistenceError {}
