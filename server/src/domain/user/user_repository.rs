use uuid::Uuid;

use crate::domain::user::User;

#[async_trait::async_trait]
pub(crate) trait UserRepository {
    async fn store(&self, user: &User) -> Result<(), StoreUserError>;

    async fn update(&self, user: &User) -> Result<(), UpdateUserError>;

    async fn get(&self, id: Uuid) -> Result<User, GetUserError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GetUserError {
    #[error(transparent)]
    NotFound(#[from] UserNotFoundError),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find user with id: {0}")]
pub(crate) struct UserNotFoundError(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum StoreUserError {
    #[error(transparent)]
    AlreadyExists(#[from] UserWithIdAlreadyExists),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("A user with id {0} already exists in the repository")]
pub(crate) struct UserWithIdAlreadyExists(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum UpdateUserError {
    #[error(transparent)]
    NotFound(#[from] UserNotFoundError),
}
