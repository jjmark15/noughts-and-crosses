use uuid::Uuid;

use crate::domain::game::Game;

#[async_trait::async_trait]
pub(crate) trait GameRepository {
    async fn get(&self, game_id: Uuid) -> Result<Game, GamePersistenceError>;

    async fn store(&self, game: &Game) -> Result<(), GamePersistenceError>;

    async fn update(&self, game: &Game) -> Result<(), GamePersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GamePersistenceError {
    #[error("Could not find game with id: {0}")]
    NotFound(Uuid),
}
