#[derive(Debug, Copy, Clone, thiserror::Error, Default)]
#[error("Exceeded player count limit")]
pub(crate) struct PlayerCountExceededError;
