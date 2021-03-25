#[derive(Debug, thiserror::Error)]
#[error("Position is already occupied")]
pub(crate) struct PositionIsAlreadyOccupiedError;

#[derive(Debug, thiserror::Error)]
#[error("Position is out of bounds")]
pub(crate) struct PositionOutOfBoundsError;

#[derive(Debug, thiserror::Error)]
#[error("Player attempted to move out of turn")]
pub(crate) struct AttemptedMoveOutOfTurnError;
