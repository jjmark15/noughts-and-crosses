use crate::domain::room::{
    GameAssignmentError, NewGameError as DomainNewGameError, RoomAssignmentError,
    RoomPersistenceError as DomainRoomPersistenceError,
};
use crate::domain::user::UserPersistenceError as DomainUserPersistenceError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct RoomCreationError {
    #[from]
    cause: DomainRoomPersistenceError,
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

#[derive(Debug, thiserror::Error)]
pub(crate) enum JoinRoomError {
    #[error(transparent)]
    RoomAssignment(#[from] RoomAssignmentError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LeaveRoomError {
    #[error(transparent)]
    RoomAssignment(#[from] RoomAssignmentError),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct NewGameError {
    #[from]
    cause: DomainNewGameError,
}

impl NewGameError {
    pub(crate) fn cause(&self) -> &DomainNewGameError {
        &self.cause
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct BecomePlayerError {
    #[from]
    cause: GameAssignmentError,
}

impl BecomePlayerError {
    pub(crate) fn cause(&self) -> &GameAssignmentError {
        &self.cause
    }
}
