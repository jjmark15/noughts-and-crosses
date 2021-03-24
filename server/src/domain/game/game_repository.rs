use uuid::Uuid;

use crate::domain::game::{Game, GameNotFoundError};

#[async_trait::async_trait]
pub(crate) trait GameRepository {
    async fn get(&self, game_id: Uuid) -> Result<Game, GamePersistenceError>;

    async fn store(&self, game: &Game) -> Result<(), GamePersistenceError>;

    async fn update(&self, game: &Game) -> Result<(), GamePersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GamePersistenceError {
    #[error(transparent)]
    NotFound(#[from] GameNotFoundError),
}
