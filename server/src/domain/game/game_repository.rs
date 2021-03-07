use crate::domain::game::Game;

#[async_trait::async_trait]
pub(crate) trait GameRepository {
    async fn store(&self, game: &Game) -> Result<(), GamePersistenceError>;
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub(crate) enum GamePersistenceError {}
