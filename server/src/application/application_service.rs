use uuid::Uuid;

use crate::application::error::RoomPersistenceError;
use crate::application::UserPersistenceError;
use crate::domain::room::{RoomFactory, RoomRepository};
use crate::domain::user::{UserFactory, UserRepository};

#[derive(Copy, Clone)]
pub(crate) struct ApplicationService<
    RR: RoomRepository,
    RF: RoomFactory,
    UR: UserRepository,
    UF: UserFactory,
> {
    room_repository: RR,
    room_factory: RF,
    user_repository: UR,
    user_factory: UF,
}

impl<RR: RoomRepository, RF: RoomFactory, UR: UserRepository, UF: UserFactory>
    ApplicationService<RR, RF, UR, UF>
{
    pub(crate) fn new(
        room_repository: RR,
        room_factory: RF,
        user_repository: UR,
        user_factory: UF,
    ) -> Self {
        ApplicationService {
            room_repository,
            room_factory,
            user_repository,
            user_factory,
        }
    }

    pub(crate) async fn create_game_room(&self) -> Result<Uuid, RoomPersistenceError> {
        let room = self.room_factory.create();
        self.room_repository.store(&room).await?;
        Ok(room.id())
    }

    pub(crate) async fn register_user<S: AsRef<str>>(
        &self,
        user_name: S,
    ) -> Result<Uuid, UserPersistenceError> {
        let user = self.user_factory.create(user_name);
        self.user_repository.store(&user).await?;
        Ok(user.id())
    }
}
