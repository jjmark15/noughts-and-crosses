use std::collections::HashSet;

use uuid::Uuid;

pub(crate) use game_repository::*;

mod game_repository;

pub(crate) struct Game {
    id: Uuid,
    players: HashSet<Uuid>,
}

impl Game {
    pub(crate) fn new(id: Uuid, players: HashSet<Uuid>) -> Self {
        Game { id, players }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn players(&self) -> &HashSet<Uuid> {
        &self.players
    }

    pub(crate) fn add_player(&mut self, user_id: Uuid) -> Result<Option<()>, GameError> {
        if self.players.len() == 2 {
            Err(GameError::PlayerCountExceeded)
        } else if self.players.insert(user_id) {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn remove_player(&mut self, user_id: Uuid) -> Option<()> {
        if self.players.remove(&user_id) {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GameError {
    #[error("Exceeded player count limit")]
    PlayerCountExceeded,
}
