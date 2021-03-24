use std::collections::HashSet;

use uuid::Uuid;

pub(crate) use error::*;
pub(crate) use game_move::*;
pub(crate) use game_play_service::*;
pub(crate) use game_repository::*;

mod error;
mod game_move;
mod game_play_service;
mod game_repository;

pub(crate) struct Game {
    id: Uuid,
    players: HashSet<Uuid>,
    moves: Vec<GameMove>,
}

impl Game {
    pub(crate) fn new(id: Uuid, players: HashSet<Uuid>, moves: Vec<GameMove>) -> Self {
        Game { id, players, moves }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn players(&self) -> &HashSet<Uuid> {
        &self.players
    }

    pub(crate) fn add_player(
        &mut self,
        user_id: Uuid,
    ) -> Result<Option<()>, PlayerCountExceededError> {
        if self.players.len() == 2 {
            Err(PlayerCountExceededError::default())
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

    pub(crate) fn moves(&self) -> &Vec<GameMove> {
        &self.moves
    }

    pub(crate) fn append_move(&mut self, game_move: GameMove) {
        self.moves.push(game_move)
    }
}
