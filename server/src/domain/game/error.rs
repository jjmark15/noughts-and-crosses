use uuid::Uuid;

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find game with id: {0}")]
pub(crate) struct GameNotFoundError(pub(crate) Uuid);
