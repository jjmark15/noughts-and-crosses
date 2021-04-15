use crate::domain::room::{
    AddPlayerError, JoinRoomError as DomainJoinRoomError, LeaveRoomError as DomainLeaveRoomError,
    NewGameError as DomainNewGameError, StoreRoomError,
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
pub(crate) struct RegisterUserError {
    #[from]
    cause: StoreUserError,
}

impl RegisterUserError {
    pub(crate) fn cause(&self) -> &StoreUserError {
        &self.cause
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct JoinRoomError {
    #[from]
    cause: DomainJoinRoomError,
}

impl JoinRoomError {
    pub(crate) fn cause(&self) -> &DomainJoinRoomError {
        &self.cause
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct LeaveRoomError {
    #[from]
    cause: DomainLeaveRoomError,
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
