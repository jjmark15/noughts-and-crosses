use uuid::Uuid;

pub(crate) use game_repository::*;

mod game_repository;

pub(crate) struct Game {
    id: Uuid,
}

impl Game {
    pub(crate) fn new(id: Uuid) -> Self {
        Game { id }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }
}
