use uuid::Uuid;

use crate::domain::game::{
    GameNotFoundError, GamePlayServiceError, GetGameError, PlayerCountExceededError,
    UpdateGameError,
};
use crate::domain::room::{GetRoomError, RoomNotFoundError, UpdateRoomError};
use crate::domain::user::{UserNotFoundError, UserPersistenceError};

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
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
}

impl From<UserPersistenceError> for NewGameError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(user_not_found_error) => user_not_found_error.into(),
        }
    }
}

impl From<GetRoomError> for NewGameError {
    fn from(err: GetRoomError) -> Self {
        match err {
            GetRoomError::NotFound(room_not_found_error) => room_not_found_error.into(),
        }
    }
}

impl From<UpdateRoomError> for NewGameError {
    fn from(err: UpdateRoomError) -> Self {
        match err {
            UpdateRoomError::NotFound(room_not_found_err) => {
                NewGameError::RoomNotFound(room_not_found_err)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RoomAssignmentError {
    #[error("Users cannot be assigned to multiple rooms")]
    AlreadyAssigned,
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
}

impl From<UserPersistenceError> for RoomAssignmentError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(user_not_found_error) => user_not_found_error.into(),
        }
    }
}

impl From<GetRoomError> for RoomAssignmentError {
    fn from(err: GetRoomError) -> Self {
        match err {
            GetRoomError::NotFound(room_not_found_error) => room_not_found_error.into(),
        }
    }
}

impl From<UpdateRoomError> for RoomAssignmentError {
    fn from(err: UpdateRoomError) -> Self {
        match err {
            UpdateRoomError::NotFound(err) => RoomAssignmentError::RoomNotFound(err),
        }
    }
}

impl From<GetGameError> for RoomAssignmentError {
    fn from(err: GetGameError) -> Self {
        match err {
            GetGameError::NotFound(game_not_found_error) => game_not_found_error.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AddPlayerError {
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
    #[error(transparent)]
    NoActiveGameInRoom(#[from] NoActiveGameInRoomError),
    #[error(transparent)]
    PlayerCountExceeded(#[from] PlayerCountExceededError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
}

impl From<UserPersistenceError> for AddPlayerError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(user_not_found_error) => user_not_found_error.into(),
        }
    }
}

impl From<GetRoomError> for AddPlayerError {
    fn from(err: GetRoomError) -> Self {
        match err {
            GetRoomError::NotFound(room_not_found_error) => room_not_found_error.into(),
        }
    }
}

impl From<GetGameError> for AddPlayerError {
    fn from(err: GetGameError) -> Self {
        match err {
            GetGameError::NotFound(game_not_found_error) => game_not_found_error.into(),
        }
    }
}

impl From<UpdateGameError> for AddPlayerError {
    fn from(err: UpdateGameError) -> Self {
        match err {
            UpdateGameError::NotFound(game_not_found) => {
                AddPlayerError::GameNotFound(game_not_found)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RemovePlayerError {
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
}

impl From<GetGameError> for RemovePlayerError {
    fn from(err: GetGameError) -> Self {
        match err {
            GetGameError::NotFound(game_not_found_err) => {
                RemovePlayerError::GameNotFound(game_not_found_err)
            }
        }
    }
}

impl From<UpdateGameError> for RemovePlayerError {
    fn from(err: UpdateGameError) -> Self {
        match err {
            UpdateGameError::NotFound(game_not_found_err) => {
                RemovePlayerError::GameNotFound(game_not_found_err)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RemoveUserError {
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
}

impl From<UserPersistenceError> for RemoveUserError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(user_not_found_err) => {
                RemoveUserError::UserNotFound(user_not_found_err)
            }
        }
    }
}

impl From<RemovePlayerError> for RemoveUserError {
    fn from(err: RemovePlayerError) -> Self {
        match err {
            RemovePlayerError::GameNotFound(game_not_found) => {
                RemoveUserError::GameNotFound(game_not_found)
            }
        }
    }
}

impl From<UpdateRoomError> for RemoveUserError {
    fn from(err: UpdateRoomError) -> Self {
        match err {
            UpdateRoomError::NotFound(room_not_found) => {
                RemoveUserError::RoomNotFound(room_not_found)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GameMoveError {
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
    #[error(transparent)]
    NoActiveGameInRoom(#[from] NoActiveGameInRoomError),
    #[error(transparent)]
    PlayerCountExceeded(#[from] PlayerCountExceededError),
    #[error(transparent)]
    UserNotInRoom(#[from] UserNotInRoomError),
    #[error(transparent)]
    GamePlayError(#[from] GamePlayServiceError),
}

impl From<UserPersistenceError> for GameMoveError {
    fn from(err: UserPersistenceError) -> Self {
        match err {
            UserPersistenceError::NotFound(user_not_found_error) => user_not_found_error.into(),
        }
    }
}

impl From<GetRoomError> for GameMoveError {
    fn from(err: GetRoomError) -> Self {
        match err {
            GetRoomError::NotFound(room_not_found_error) => room_not_found_error.into(),
        }
    }
}

impl From<GetGameError> for GameMoveError {
    fn from(err: GetGameError) -> Self {
        match err {
            GetGameError::NotFound(game_not_found_error) => game_not_found_error.into(),
        }
    }
}

impl From<UpdateGameError> for GameMoveError {
    fn from(err: UpdateGameError) -> Self {
        match err {
            UpdateGameError::NotFound(game_not_found_error) => game_not_found_error.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("There is no currently active game for room with id: {0}")]
pub(crate) struct NoActiveGameInRoomError(pub(crate) Uuid);
