use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::Mutex;
use uuid::Uuid;

use crate::domain::game::{
    Game, GameMove, GameMovePosition, GameNotFoundError, GameRepository, GameWithIdAlreadyExists,
    GetGameError, StoreGameError, UpdateGameError,
};

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
    async fn get(&self, game_id: Uuid) -> Result<Game, GetGameError> {
        let map = self.inner.lock();
        let stored_game = map
            .get(&game_id)
            .ok_or_else::<GetGameError, _>(|| GameNotFoundError(game_id).into())?;
        Ok(from_stored_game(game_id, stored_game))
    }

    async fn store(&self, game: &Game) -> Result<(), StoreGameError> {
        let mut map = self.inner.lock();
        if map.insert(game.id(), game.into()).is_some() {
            return Err(GameWithIdAlreadyExists(game.id()).into());
        }
        Ok(())
    }

    async fn update(&self, game: &Game) -> Result<(), UpdateGameError> {
        let game_id = game.id();
        let mut map = self.inner.lock();
        let _stored_game = map
            .get(&game_id)
            .ok_or_else::<UpdateGameError, _>(|| GameNotFoundError(game_id).into())?;
        map.insert(game_id, game.into());
        Ok(())
    }
}

struct StoredGame {
    players: HashSet<Uuid>,
    moves: Vec<StoredGameMove>,
}

#[derive(Debug, Copy, Clone)]
struct StoredGameMove {
    user_id: Uuid,
    position: StoredGameMovePosition,
}

impl From<&GameMove> for StoredGameMove {
    fn from(game_move: &GameMove) -> Self {
        StoredGameMove {
            user_id: game_move.user_id(),
            position: game_move.position().into(),
        }
    }
}

impl From<&StoredGameMove> for GameMove {
    fn from(game_move: &StoredGameMove) -> Self {
        GameMove::new(game_move.user_id, game_move.position.into())
    }
}

#[derive(Debug, Copy, Clone)]
struct StoredGameMovePosition {
    x: u8,
    y: u8,
}

impl From<GameMovePosition> for StoredGameMovePosition {
    fn from(position: GameMovePosition) -> Self {
        StoredGameMovePosition {
            x: position.x(),
            y: position.y(),
        }
    }
}

impl From<StoredGameMovePosition> for GameMovePosition {
    fn from(position: StoredGameMovePosition) -> Self {
        GameMovePosition::new(position.x, position.y)
    }
}

impl From<&Game> for StoredGame {
    fn from(game: &Game) -> Self {
        let moves = game.moves().iter().map(StoredGameMove::from).collect();
        StoredGame {
            players: game.players().clone(),
            moves,
        }
    }
}

fn from_stored_game(id: Uuid, stored_game: &StoredGame) -> Game {
    Game::new(
        id,
        stored_game.players.clone(),
        stored_game.moves.iter().map(GameMove::from).collect(),
    )
}
