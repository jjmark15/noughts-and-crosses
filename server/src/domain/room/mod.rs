use uuid::Uuid;

pub(crate) use room_factory::*;
pub(crate) use room_manager::*;
pub(crate) use room_repository::*;

mod room_factory;
mod room_manager;
mod room_repository;

pub(crate) struct Room {
    id: Uuid,
    active_game_id: Option<Uuid>,
}

impl Room {
    pub(crate) fn new(id: Uuid, active_game_id: Option<Uuid>) -> Self {
        Room { id, active_game_id }
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
}
