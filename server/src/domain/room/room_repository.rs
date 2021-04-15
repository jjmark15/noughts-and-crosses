use uuid::Uuid;

use crate::domain::room::Room;
use crate::domain::user::User;

#[async_trait::async_trait]
pub(crate) trait RoomRepository {
    async fn store(&self, room: &Room) -> Result<(), StoreRoomError>;

    async fn update(&self, room: &Room) -> Result<(), UpdateRoomError>;

    async fn get(&self, room_id: Uuid) -> Result<Room, GetRoomError>;

    async fn have_member(&self, user: &User) -> Vec<Room>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GetRoomError {
    #[error(transparent)]
    NotFound(#[from] RoomNotFoundError),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find room with id: {0}")]
pub(crate) struct RoomNotFoundError(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum StoreRoomError {
    #[error(transparent)]
    AlreadyExists(#[from] RoomWithIdAlreadyExists),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("A room with id {0} already exists in the repository")]
pub(crate) struct RoomWithIdAlreadyExists(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum UpdateRoomError {
    #[error(transparent)]
    NotFound(#[from] RoomNotFoundError),
}
