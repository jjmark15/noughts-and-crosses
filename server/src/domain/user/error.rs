use uuid::Uuid;

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find user with id: {0}")]
pub(crate) struct UserNotFoundError(pub(crate) Uuid);
