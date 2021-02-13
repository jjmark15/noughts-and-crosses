use uuid::Uuid;

pub(crate) use room_factory::*;
pub(crate) use room_repository::*;

mod room_factory;
mod room_repository;

pub(crate) struct Room {
    id: Uuid,
}

impl Room {
    pub(crate) fn new(id: Uuid) -> Self {
        Room { id }
    }

    pub(crate) fn id(&self) -> &Uuid {
        &self.id
    }
}
