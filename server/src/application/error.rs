use crate::domain::room::{
    AddPlayerError, NewGameError as DomainNewGameError, RemoveUserError, RoomAssignmentError,
    StoreRoomError,
};
use crate::domain::user::StoreUserError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct RoomCreationError {
    #[from]
    cause: StoreRoomError,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct AddUserError {
    #[from]
    cause: StoreUserError,
}

impl AddUserError {
    pub(crate) fn cause(&self) -> &StoreUserError {
        &self.cause
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
    RemoveUser(#[from] RemoveUserError),
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
    cause: AddPlayerError,
}

impl BecomePlayerError {
    pub(crate) fn cause(&self) -> &AddPlayerError {
        &self.cause
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct GameMoveError {
    #[from]
    cause: crate::domain::room::GameMoveError,
}

impl GameMoveError {
    pub(crate) fn cause(&self) -> &crate::domain::room::GameMoveError {
        &self.cause
    }
}
