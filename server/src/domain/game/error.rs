use uuid::Uuid;

#[derive(Debug, Copy, Clone, thiserror::Error, Default)]
#[error("Exceeded player count limit")]
pub(crate) struct PlayerCountExceededError;

#[derive(Debug, Copy, Clone, thiserror::Error, Default)]
#[error("User({0}) is not a player in game")]
pub(crate) struct UserNotAPlayerInGameError(pub(crate) Uuid);
