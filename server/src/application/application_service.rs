use uuid::Uuid;

use crate::application::error::RoomPersistenceError;
use crate::application::UserPersistenceError;
use crate::domain::room::{RoomFactory, RoomRepository};
use crate::domain::user::{UserFactory, UserRepository};

#[async_trait::async_trait]
pub(crate) trait ApplicationService {
    async fn create_game_room(&self) -> Result<Uuid, RoomPersistenceError>;

    async fn register_user(&self, user_name: String) -> Result<Uuid, UserPersistenceError>;

    async fn get_user_name(&self, user_id: Uuid) -> Result<String, UserPersistenceError>;
}

#[derive(Copy, Clone)]
pub(crate) struct ApplicationServiceImpl<
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
    ApplicationServiceImpl<RR, RF, UR, UF>
{
    pub(crate) fn new(
        room_repository: RR,
        room_factory: RF,
        user_repository: UR,
        user_factory: UF,
    ) -> Self {
        ApplicationServiceImpl {
            room_repository,
            room_factory,
            user_repository,
            user_factory,
        }
    }
}

#[async_trait::async_trait]
impl<RR, RF, UR, UF> ApplicationService for ApplicationServiceImpl<RR, RF, UR, UF>
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
    UR: UserRepository + Send + Sync,
    UF: UserFactory + Send + Sync,
{
    async fn create_game_room(&self) -> Result<Uuid, RoomPersistenceError> {
        let room = self.room_factory.create();
        self.room_repository.store(&room).await?;
        Ok(room.id())
    }

    async fn register_user(&self, user_name: String) -> Result<Uuid, UserPersistenceError> {
        let user = self.user_factory.create(user_name);
        self.user_repository.store(&user).await?;
        Ok(user.id())
    }

    async fn get_user_name(&self, user_id: Uuid) -> Result<String, UserPersistenceError> {
        let user = self.user_repository.get(user_id).await?;

        Ok(user.name().to_string())
    }
}
