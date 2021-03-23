use uuid::Uuid;

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find user with id: {user_id}")]
pub(crate) struct UserNotFoundError {
    user_id: Uuid,
}

impl UserNotFoundError {
    pub(crate) fn new(user_id: Uuid) -> Self {
        UserNotFoundError { user_id }
    }
}
