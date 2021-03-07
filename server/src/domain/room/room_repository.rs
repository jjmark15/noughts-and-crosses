use uuid::Uuid;

use crate::domain::room::Room;

#[async_trait::async_trait]
pub(crate) trait RoomRepository {
    async fn store(&self, room: &Room) -> Result<(), RoomPersistenceError>;

    async fn update(&self, room: &Room) -> Result<(), RoomPersistenceError>;

    async fn get(&self, room_id: Uuid) -> Result<Room, RoomPersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum RoomPersistenceError {
    #[error("Could not find room with id: {0}")]
    RoomNotFound(Uuid),
}
