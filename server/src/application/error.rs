use crate::domain::room::RoomPersistenceError as DomainRoomPersistenceError;

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
