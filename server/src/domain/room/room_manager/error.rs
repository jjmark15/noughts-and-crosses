use uuid::Uuid;

use crate::domain::game::{GameError, GamePersistenceError, GamePlayServiceError};
use crate::domain::room::RoomPersistenceError;
use crate::domain::user::UserPersistenceError;

#[derive(Debug, thiserror::Error)]
#[error("User({user_id}) is not a member of Room({room_id})")]
pub(crate) struct UserNotInRoomError {
    user_id: Uuid,
    room_id: Uuid,
}

impl UserNotInRoomError {
    pub(crate) fn new(user_id: Uuid, room_id: Uuid) -> Self {
        UserNotInRoomError { user_id, room_id }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum NewGameError {
    #[error(transparent)]
    UserNotFound(UserPersistenceError),
    #[error(transparent)]
    RoomNotFound(RoomPersistenceError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
}

impl From<UserPersistenceError> for NewGameError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(_) => NewGameError::UserNotFound(err),
        }
    }
}

impl From<RoomPersistenceError> for NewGameError {
    fn from(err: RoomPersistenceError) -> Self {
        match err {
            RoomPersistenceError::NotFound(_) => NewGameError::RoomNotFound(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RoomAssignmentError {
    #[error("Users cannot be assigned to multiple rooms")]
    AlreadyAssigned,
    #[error(transparent)]
    UserNotFound(UserPersistenceError),
    #[error(transparent)]
    RoomNotFound(RoomPersistenceError),
    #[error(transparent)]
    GameNotFound(GamePersistenceError),
}

impl From<UserPersistenceError> for RoomAssignmentError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(_) => RoomAssignmentError::UserNotFound(err),
        }
    }
}

impl From<RoomPersistenceError> for RoomAssignmentError {
    fn from(err: RoomPersistenceError) -> Self {
        match err {
            RoomPersistenceError::NotFound(_) => RoomAssignmentError::RoomNotFound(err),
        }
    }
}

impl From<GamePersistenceError> for RoomAssignmentError {
    fn from(err: GamePersistenceError) -> Self {
        match err {
            GamePersistenceError::NotFound(_) => RoomAssignmentError::GameNotFound(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GameAssignmentError {
    #[error(transparent)]
    UserNotFound(UserPersistenceError),
    #[error(transparent)]
    RoomNotFound(RoomPersistenceError),
    #[error(transparent)]
    GameNotFound(GamePersistenceError),
    #[error("There is no currently active game for room with id: {0}")]
    NoActiveGameInRoom(Uuid),
    #[error(transparent)]
    PlayerCountExceeded(GameError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
}

impl From<UserPersistenceError> for GameAssignmentError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(_) => GameAssignmentError::UserNotFound(err),
        }
    }
}

impl From<RoomPersistenceError> for GameAssignmentError {
    fn from(err: RoomPersistenceError) -> Self {
        match err {
            RoomPersistenceError::NotFound(_) => GameAssignmentError::RoomNotFound(err),
        }
    }
}

impl From<GameError> for GameAssignmentError {
    fn from(err: GameError) -> Self {
        match err {
            GameError::PlayerCountExceeded => GameAssignmentError::PlayerCountExceeded(err),
        }
    }
}

impl From<GamePersistenceError> for GameAssignmentError {
    fn from(err: GamePersistenceError) -> Self {
        match err {
            GamePersistenceError::NotFound(_) => GameAssignmentError::GameNotFound(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GameMoveError {
    #[error(transparent)]
    UserNotFound(UserPersistenceError),
    #[error(transparent)]
    RoomNotFound(RoomPersistenceError),
    #[error(transparent)]
    GameNotFound(GamePersistenceError),
    #[error("There is no currently active game for room with id: {0}")]
    NoActiveGameInRoom(Uuid),
    #[error(transparent)]
    PlayerCountExceeded(GameError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
    #[error(transparent)]
    GamePlayError(#[from] GamePlayServiceError),
}

impl From<UserPersistenceError> for GameMoveError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(_) => GameMoveError::UserNotFound(err),
        }
    }
}

impl From<RoomPersistenceError> for GameMoveError {
    fn from(err: RoomPersistenceError) -> Self {
        match err {
            RoomPersistenceError::NotFound(_) => GameMoveError::RoomNotFound(err),
        }
    }
}

impl From<GameError> for GameMoveError {
    fn from(err: GameError) -> Self {
        match err {
            GameError::PlayerCountExceeded => GameMoveError::PlayerCountExceeded(err),
        }
    }
}

impl From<GamePersistenceError> for GameMoveError {
    fn from(err: GamePersistenceError) -> Self {
        match err {
            GamePersistenceError::NotFound(_) => GameMoveError::GameNotFound(err),
        }
    }
}
