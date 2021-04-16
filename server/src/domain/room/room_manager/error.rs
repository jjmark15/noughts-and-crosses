use uuid::Uuid;

use crate::domain::game::{
    ApplyMoveError, GameNotFoundError, GetGameError, PlayerCountExceededError, UpdateGameError,
};
use crate::domain::room::{GetRoomError, RoomNotFoundError, UpdateRoomError};
use crate::domain::user::{GetUserError, UserNotFoundError};

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

impl From<GetUserError> for NewGameError {
    fn from(err: GetUserError) -> Self {
        match err {
            GetUserError::NotFound(user_not_found_error) => user_not_found_error.into(),
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
pub(crate) enum JoinRoomError {
    #[error("Users cannot be assigned to multiple rooms")]
    AlreadyAssigned,
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
}

impl From<GetUserError> for JoinRoomError {
    fn from(err: GetUserError) -> Self {
        match err {
            GetUserError::NotFound(user_not_found_error) => user_not_found_error.into(),
        }
    }
}

impl From<GetRoomError> for JoinRoomError {
    fn from(err: GetRoomError) -> Self {
        match err {
            GetRoomError::NotFound(room_not_found_error) => room_not_found_error.into(),
        }
    }
}

impl From<UpdateRoomError> for JoinRoomError {
    fn from(err: UpdateRoomError) -> Self {
        match err {
            UpdateRoomError::NotFound(err) => JoinRoomError::RoomNotFound(err),
        }
    }
}

impl From<GetGameError> for JoinRoomError {
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

impl From<GetUserError> for AddPlayerError {
    fn from(err: GetUserError) -> Self {
        match err {
            GetUserError::NotFound(user_not_found_error) => user_not_found_error.into(),
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
pub(crate) enum LeaveRoomError {
    #[error(transparent)]
    UserNotFound(#[from] UserNotFoundError),
    #[error(transparent)]
    GameNotFound(#[from] GameNotFoundError),
    #[error(transparent)]
    RoomNotFound(#[from] RoomNotFoundError),
}

impl From<GetUserError> for LeaveRoomError {
    fn from(err: GetUserError) -> Self {
        match err {
            GetUserError::NotFound(user_not_found_err) => {
                LeaveRoomError::UserNotFound(user_not_found_err)
            }
        }
    }
}

impl From<RemovePlayerError> for LeaveRoomError {
    fn from(err: RemovePlayerError) -> Self {
        match err {
            RemovePlayerError::GameNotFound(game_not_found) => {
                LeaveRoomError::GameNotFound(game_not_found)
            }
        }
    }
}

impl From<UpdateRoomError> for LeaveRoomError {
    fn from(err: UpdateRoomError) -> Self {
        match err {
            UpdateRoomError::NotFound(room_not_found) => {
                LeaveRoomError::RoomNotFound(room_not_found)
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
    GamePlayError(#[from] ApplyMoveError),
}

impl From<GetUserError> for GameMoveError {
    fn from(err: GetUserError) -> Self {
        match err {
            GetUserError::NotFound(user_not_found_error) => user_not_found_error.into(),
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
