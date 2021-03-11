use std::sync::Arc;

use uuid::Uuid;

use crate::application::error::RoomCreationError;
use crate::application::{JoinRoomError, LeaveRoomError, NewGameError, UserPersistenceError};
use crate::domain::room::{RoomFactory, RoomManager, RoomRepository};
use crate::domain::user::{UserFactory, UserRepository};

#[async_trait::async_trait]
pub(crate) trait ApplicationService {
    async fn register_user(&self, user_name: String) -> Result<Uuid, UserPersistenceError>;

    async fn get_user_name(&self, user_id: Uuid) -> Result<String, UserPersistenceError>;

    async fn create_room(&self) -> Result<Uuid, RoomCreationError>;

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), NewGameError>;

    async fn join_room(&self, room_id: Uuid, user_id: Uuid) -> Result<(), JoinRoomError>;

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError>;
}

#[derive(Clone)]
pub(crate) struct ApplicationServiceImpl<
    RR: RoomRepository,
    RF: RoomFactory,
    UR: UserRepository,
    UF: UserFactory,
    RM: RoomManager,
> {
    room_repository: Arc<RR>,
    room_factory: RF,
    user_repository: Arc<UR>,
    user_factory: UF,
    room_manager: RM,
}

impl<RR: RoomRepository, RF: RoomFactory, UR: UserRepository, UF: UserFactory, RM: RoomManager>
    ApplicationServiceImpl<RR, RF, UR, UF, RM>
{
    pub(crate) fn new(
        room_repository: Arc<RR>,
        room_factory: RF,
        user_repository: Arc<UR>,
        user_factory: UF,
        room_manager: RM,
    ) -> Self {
        ApplicationServiceImpl {
            room_repository,
            room_factory,
            user_repository,
            user_factory,
            room_manager,
        }
    }
}

#[async_trait::async_trait]
impl<RR, RF, UR, UF, RM> ApplicationService for ApplicationServiceImpl<RR, RF, UR, UF, RM>
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
    UR: UserRepository + Send + Sync,
    UF: UserFactory + Send + Sync,
    RM: RoomManager + Send + Sync,
{
    async fn register_user(&self, user_name: String) -> Result<Uuid, UserPersistenceError> {
        let user = self.user_factory.create(user_name);
        self.user_repository.store(&user).await?;
        Ok(user.id())
    }

    async fn get_user_name(&self, user_id: Uuid) -> Result<String, UserPersistenceError> {
        let user = self.user_repository.get(user_id).await?;

        Ok(user.name().to_string())
    }

    async fn create_room(&self) -> Result<Uuid, RoomCreationError> {
        let room = self.room_factory.create();
        self.room_repository.store(&room).await?;
        Ok(room.id())
    }

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), NewGameError> {
        self.room_manager
            .start_new_game(room_id, user_id)
            .await
            .map_err(NewGameError::from)
    }

    async fn join_room(&self, room_id: Uuid, user_id: Uuid) -> Result<(), JoinRoomError> {
        self.room_manager
            .assign_user(user_id, room_id)
            .await
            .map_err(JoinRoomError::from)
    }

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError> {
        self.room_manager
            .unassign_user(user_id)
            .await
            .map_err(LeaveRoomError::from)
    }
}
