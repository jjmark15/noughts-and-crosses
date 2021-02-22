use std::sync::Arc;

use uuid::Uuid;

use crate::domain::user::{User, UserPersistenceError, UserRepository};

#[async_trait::async_trait]
pub(crate) trait RoomAssignmentService {
    async fn assign_user(&self, user_id: Uuid, room_id: Uuid) -> Result<(), RoomAssignmentError>;

    async fn unassign_user(&self, user_id: Uuid) -> Result<(), RoomAssignmentError>;
}

pub(crate) struct RoomAssignmentServiceImpl<UR>
where
    UR: UserRepository,
{
    user_repository: Arc<UR>,
}

impl<UR: UserRepository> RoomAssignmentServiceImpl<UR> {
    pub(crate) fn new(user_repository: Arc<UR>) -> Self {
        RoomAssignmentServiceImpl { user_repository }
    }
}

#[async_trait::async_trait]
impl<UR> RoomAssignmentService for RoomAssignmentServiceImpl<UR>
where
    UR: UserRepository + Send + Sync,
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
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RoomAssignmentError {
    #[error("Users cannot be assigned to multiple rooms")]
    AlreadyAssigned,
    #[error(transparent)]
    UserPersistence(#[from] UserPersistenceError),
}
