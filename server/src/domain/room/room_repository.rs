use crate::domain::room::Room;

#[async_trait::async_trait]
pub(crate) trait RoomRepository {
    async fn store(&self, room: &Room) -> Result<(), RoomPersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum RoomPersistenceError {}
