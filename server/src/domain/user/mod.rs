use uuid::Uuid;

pub(crate) use error::*;
pub(crate) use user_client::*;
pub(crate) use user_factory::*;
pub(crate) use user_repository::*;

mod error;
mod user_client;
mod user_factory;
mod user_repository;

pub(crate) struct User {
    id: Uuid,
    name: String,
}

impl User {
    pub(crate) fn new(id: uuid::Uuid, name: String) -> Self {
        User { id, name }
    }

    pub(crate) fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
