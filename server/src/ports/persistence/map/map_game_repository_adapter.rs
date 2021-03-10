use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::Mutex;
use uuid::Uuid;

use crate::domain::game::{Game, GamePersistenceError, GameRepository};

type EmbeddedDb = Arc<Mutex<HashMap<Uuid, StoredGame>>>;

pub(crate) struct MapGameRepositoryAdapter {
    inner: EmbeddedDb,
}

impl MapGameRepositoryAdapter {
    pub(crate) fn new() -> Self {
        MapGameRepositoryAdapter {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl GameRepository for MapGameRepositoryAdapter {
    async fn get(&self, game_id: Uuid) -> Result<Game, GamePersistenceError> {
        let map = self.inner.lock();
        let stored_game = map
            .get(&game_id)
            .ok_or(GamePersistenceError::NotFound(game_id))?;
        Ok(from_stored_game(game_id, stored_game))
    }

    async fn store(&self, game: &Game) -> Result<(), GamePersistenceError> {
        let mut map = self.inner.lock();
        map.insert(game.id(), game.into());
        Ok(())
    }

    async fn update(&self, game: &Game) -> Result<(), GamePersistenceError> {
        let game_id = game.id();
        let mut map = self.inner.lock();
        let _stored_game = map
            .get(&game_id)
            .ok_or(GamePersistenceError::NotFound(game_id))?;
        map.insert(game_id, game.into());
        Ok(())
    }
}

struct StoredGame {
    players: HashSet<Uuid>,
}

impl From<&Game> for StoredGame {
    fn from(game: &Game) -> Self {
        StoredGame {
            players: game.players().clone(),
        }
    }
}

fn from_stored_game(id: Uuid, stored_game: &StoredGame) -> Game {
    Game::new(id, stored_game.players.clone())
}
