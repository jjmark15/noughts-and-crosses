use std::sync::Arc;

use uuid::Uuid;

use crate::application::error::RoomCreationError;
use crate::application::{JoinRoomError, LeaveRoomError, UserPersistenceError};
use crate::domain::room::{RoomFactory, RoomRepository};
use crate::domain::user::{UserClientProvider, UserFactory, UserRepository};
use crate::domain::RoomAssignmentService;

#[async_trait::async_trait]
pub(crate) trait ApplicationService {
    async fn register_user(&self, user_name: String) -> Result<Uuid, UserPersistenceError>;

    async fn get_user_name(&self, user_id: Uuid) -> Result<String, UserPersistenceError>;

    async fn create_room(&self) -> Result<Uuid, RoomCreationError>;

    async fn join_room(&self, room_id: Uuid, user_id: Uuid) -> Result<(), JoinRoomError>;

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError>;
}

#[derive(Clone)]
pub(crate) struct ApplicationServiceImpl<
    RR: RoomRepository,
    RF: RoomFactory,
    UR: UserRepository,
    UF: UserFactory,
    UCP: UserClientProvider,
    RAS: RoomAssignmentService,
> {
    room_repository: RR,
    room_factory: RF,
    user_repository: Arc<UR>,
    user_factory: UF,
    user_client_provider: Arc<UCP>,
    room_assignment_service: RAS,
}

impl<
        RR: RoomRepository,
        RF: RoomFactory,
        UR: UserRepository,
        UF: UserFactory,
        UCP: UserClientProvider,
        RAS: RoomAssignmentService,
    > ApplicationServiceImpl<RR, RF, UR, UF, UCP, RAS>
{
    pub(crate) fn new(
        room_repository: RR,
        room_factory: RF,
        user_repository: Arc<UR>,
        user_factory: UF,
        user_client_provider: Arc<UCP>,
        room_assignment_service: RAS,
    ) -> Self {
        ApplicationServiceImpl {
            room_repository,
            room_factory,
            user_repository,
            user_factory,
            user_client_provider,
            room_assignment_service,
        }
    }
}

#[async_trait::async_trait]
impl<RR, RF, UR, UF, UCP, RAS> ApplicationService
    for ApplicationServiceImpl<RR, RF, UR, UF, UCP, RAS>
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
    UR: UserRepository + Send + Sync,
    UF: UserFactory + Send + Sync,
    UCP: UserClientProvider + Send + Sync,
    RAS: RoomAssignmentService + Send + Sync,
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

    async fn join_room(&self, room_id: Uuid, user_id: Uuid) -> Result<(), JoinRoomError> {
        self.room_assignment_service
            .assign_user(user_id, room_id)
            .await?;
        Ok(())
    }

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError> {
        self.room_assignment_service.unassign_user(user_id).await?;
        Ok(())
    }
}
