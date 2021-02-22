use uuid::Uuid;

pub(crate) use user_client::*;
pub(crate) use user_factory::*;
pub(crate) use user_repository::*;

mod user_client;
mod user_factory;
mod user_repository;

pub(crate) struct User {
    id: Uuid,
    name: String,
    room_id: Option<Uuid>,
}

impl User {
    pub(crate) fn new(id: uuid::Uuid, name: String, room_id: Option<Uuid>) -> Self {
        User { id, name, room_id }
    }

    pub(crate) fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn room_id(&self) -> Option<Uuid> {
        self.room_id
    }
}
