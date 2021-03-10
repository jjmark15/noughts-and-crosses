use std::sync::Arc;

use uuid::Uuid;

use crate::domain::game::{Game, GameRepository};
use crate::domain::room::{Room, RoomPersistenceError, RoomRepository};
use crate::domain::user::{User, UserPersistenceError, UserRepository};

#[async_trait::async_trait]
pub(crate) trait RoomManager {
    async fn assign_user(&self, user_id: Uuid, room_id: Uuid) -> Result<(), RoomAssignmentError>;

    async fn unassign_user(&self, user_id: Uuid) -> Result<(), RoomAssignmentError>;

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), RoomManagerError>;
}

pub(crate) struct RoomManagerImpl<UR: UserRepository, RR: RoomRepository, GR: GameRepository> {
    user_repository: Arc<UR>,
    room_repository: Arc<RR>,
    game_repository: GR,
}

impl<UR, RR, GR> RoomManagerImpl<UR, RR, GR>
where
    UR: UserRepository,
    RR: RoomRepository,
    GR: GameRepository,
{
    pub(crate) fn new(
        user_repository: Arc<UR>,
        room_repository: Arc<RR>,
        game_repository: GR,
    ) -> Self {
        RoomManagerImpl {
            user_repository,
            room_repository,
            game_repository,
        }
    }

    fn user_is_in_room(user: &User, room: &Room) -> bool {
        if let Some(id) = user.room_id() {
            return id == room.id();
        }
        false
    }
}

#[async_trait::async_trait]
impl<UR, RR, GR> RoomManager for RoomManagerImpl<UR, RR, GR>
where
    UR: UserRepository + Send + Sync,
    RR: RoomRepository + Send + Sync,
    GR: GameRepository + Send + Sync,
{
    async fn assign_user(&self, user_id: Uuid, room_id: Uuid) -> Result<(), RoomAssignmentError> {
        let user = self.user_repository.get(user_id).await?;
        if let Some(_existing_room_id) = user.room_id() {
            return Err(RoomAssignmentError::AlreadyAssigned);
        }
        let user_with_assignment = User::new(user_id, user.name().to_string(), Some(room_id));
        self.user_repository.update(&user_with_assignment).await?;
        Ok(())
    }

    async fn unassign_user(&self, user_id: Uuid) -> Result<(), RoomAssignmentError> {
        let user = self.user_repository.get(user_id).await?;
        if user.room_id().is_none() {
            return Ok(());
        }
        let user_without_assignment = User::new(user.id(), user.name().to_string(), None);
        self.user_repository
            .update(&user_without_assignment)
            .await?;
        Ok(())
    }

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), RoomManagerError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(RoomManagerError::UserNotFound)?;
        let mut room = self
            .room_repository
            .get(room_id)
            .await
            .map_err(RoomManagerError::RoomNotFound)?;

        if !Self::user_is_in_room(&user, &room) {
            return Err(RoomManagerError::UserNotInRoom(user_id, room_id));
        }

        let game = Game::new(Uuid::new_v4());
        self.game_repository.store(&game).await.unwrap();
        room.set_active_game_id(game.id());
        self.room_repository
            .update(&room)
            .await
            .map_err(room_update_error)?;

        Ok(())
    }
}

fn room_update_error(persistence_error: RoomPersistenceError) -> RoomManagerError {
    match persistence_error {
        RoomPersistenceError::RoomNotFound(_) => RoomManagerError::RoomNotFound(persistence_error),
    }
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub(crate) enum RoomManagerError {
    #[error(transparent)]
    UserNotFound(UserPersistenceError),
    #[error(transparent)]
    RoomNotFound(RoomPersistenceError),
    #[error("User({0}) is not a member of Room({1})")]
    UserNotInRoom(Uuid, Uuid),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RoomAssignmentError {
    #[error("Users cannot be assigned to multiple rooms")]
    AlreadyAssigned,
    #[error(transparent)]
    UserPersistence(#[from] UserPersistenceError),
}