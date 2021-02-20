use crate::domain::room::RoomPersistenceError as DomainRoomPersistenceError;
use crate::domain::user::UserPersistenceError as DomainUserPersistenceError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct RoomPersistenceError {
    #[from]
    cause: DomainRoomPersistenceError,
}

impl RoomPersistenceError {
    pub(crate) fn cause(&self) -> DomainRoomPersistenceError {
        self.cause
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct UserPersistenceError {
    #[from]
    cause: DomainUserPersistenceError,
}

impl UserPersistenceError {
    pub(crate) fn cause(&self) -> DomainUserPersistenceError {
        self.cause
    }
}
