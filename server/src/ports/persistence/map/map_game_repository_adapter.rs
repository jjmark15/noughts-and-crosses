use std::collections::HashMap;
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
    async fn store(&self, game: &Game) -> Result<(), GamePersistenceError> {
        let mut map = self.inner.lock();
        map.insert(game.id(), game.into());
        Ok(())
    }
}

struct StoredGame;

impl From<&Game> for StoredGame {
    fn from(_game: &Game) -> Self {
        StoredGame
    }
}
