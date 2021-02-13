use uuid::Uuid;

use crate::application::error::RoomPersistenceError;
use crate::domain::room::{RoomFactory, RoomRepository};

#[derive(Copy, Clone)]
pub(crate) struct ApplicationService<RR: RoomRepository, RF: RoomFactory> {
    room_repository: RR,
    room_factory: RF,
}

impl<RR: RoomRepository, RF: RoomFactory> ApplicationService<RR, RF> {
    pub(crate) fn new(room_repository: RR, room_factory: RF) -> Self {
        ApplicationService {
            room_repository,
            room_factory,
        }
    }

    pub(crate) async fn create_game_room(&self) -> Result<Uuid, RoomPersistenceError> {
        let room = self.room_factory.create();
        self.room_repository.store(&room).await?;
        Ok(*room.id())
    }
}
