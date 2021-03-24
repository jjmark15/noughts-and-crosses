use uuid::Uuid;

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find room with id: {0}")]
pub(crate) struct RoomNotFoundError(pub(crate) Uuid);
