use std::collections::HashSet;

use uuid::Uuid;

pub(crate) use error::*;
pub(crate) use room_factory::*;
pub(crate) use room_manager::*;
pub(crate) use room_repository::*;

mod error;
mod room_factory;
mod room_manager;
mod room_repository;

pub(crate) struct Room {
    id: Uuid,
    active_game_id: Option<Uuid>,
    members: HashSet<Uuid>,
}

impl Room {
    pub(crate) fn new(id: Uuid, active_game_id: Option<Uuid>, members: HashSet<Uuid>) -> Self {
        Room {
            id,
            active_game_id,
            members,
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn active_game_id(&self) -> Option<Uuid> {
        self.active_game_id
    }

    pub(crate) fn set_active_game_id(&mut self, game_id: Uuid) {
        self.active_game_id = Some(game_id);
    }

    pub(crate) fn members(&self) -> &HashSet<Uuid> {
        &self.members
    }

    pub(crate) fn remove_member(&mut self, user_id: Uuid) {
        self.members.remove(&user_id);
    }

    pub(crate) fn add_member(&mut self, user_id: Uuid) {
        self.members.insert(user_id);
    }

    pub(crate) fn is_member(&self, user_id: Uuid) -> bool {
        self.members.contains(&user_id)
    }
}
