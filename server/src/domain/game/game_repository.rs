use uuid::Uuid;

use crate::domain::game::Game;

#[async_trait::async_trait]
pub(crate) trait GameRepository {
    async fn get(&self, game_id: Uuid) -> Result<Game, GetGameError>;

    async fn store(&self, game: &Game) -> Result<(), StoreGameError>;

    async fn update(&self, game: &Game) -> Result<(), UpdateGameError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GetGameError {
    #[error(transparent)]
    NotFound(#[from] GameNotFoundError),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("Could not find game with id: {0}")]
pub(crate) struct GameNotFoundError(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum StoreGameError {
    #[error(transparent)]
    AlreadyExists(#[from] GameWithIdAlreadyExists),
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
#[error("A game with id {0} already exists in the repository")]
pub(crate) struct GameWithIdAlreadyExists(pub(crate) Uuid);

#[derive(Debug, thiserror::Error)]
pub(crate) enum UpdateGameError {
    #[error(transparent)]
    NotFound(#[from] GameNotFoundError),
}
