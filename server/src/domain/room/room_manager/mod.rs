use std::collections::HashSet;
use std::sync::Arc;

use uuid::Uuid;

pub(crate) use error::*;

use crate::domain::game::{Game, GamePersistenceError, GameRepository};
use crate::domain::room::{Room, RoomRepository};
use crate::domain::user::{User, UserRepository};

mod error;

#[async_trait::async_trait]
pub(crate) trait RoomManager {
    async fn assign_user(&self, user_id: Uuid, room_id: Uuid) -> Result<(), RoomAssignmentError>;

    async fn unassign_user(&self, user_id: Uuid) -> Result<(), RoomAssignmentError>;

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), NewGameError>;

    async fn add_player(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<()>, GameAssignmentError>;
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
        room.is_member(user.id())
    }

    async fn remove_player(
        &self,
        user_id: Uuid,
        game_id: Uuid,
    ) -> Result<(), GamePersistenceError> {
        let mut game = self.game_repository.get(game_id).await?;
        game.remove_player(user_id);
        self.game_repository.update(&game).await?;
        Ok(())
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
        if !self.room_repository.have_member(&user).await?.is_empty() {
            return Err(RoomAssignmentError::AlreadyAssigned);
        }
        let mut room = self.room_repository.get(room_id).await?;
        room.add_member(user.id());
        self.room_repository.update(&room).await?;
        Ok(())
    }

    async fn unassign_user(&self, user_id: Uuid) -> Result<(), RoomAssignmentError> {
        let user = self.user_repository.get(user_id).await?;
        let mut rooms = self.room_repository.have_member(&user).await?;
        rooms
            .iter_mut()
            .for_each(|room| room.remove_member(user_id));
        for room in &rooms {
            if let Some(game_id) = room.active_game_id() {
                self.remove_player(user_id, game_id).await?;
            }
            self.room_repository.update(room).await?;
        }
        Ok(())
    }

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), NewGameError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(NewGameError::from)?;
        let mut room = self
            .room_repository
            .get(room_id)
            .await
            .map_err(NewGameError::from)?;

        if !Self::user_is_in_room(&user, &room) {
            return Err(UserNotInRoomError::new(user_id, room_id).into());
        }

        let game = Game::new(Uuid::new_v4(), HashSet::new());
        self.game_repository.store(&game).await.unwrap();
        room.set_active_game_id(game.id());
        self.room_repository
            .update(&room)
            .await
            .map_err(NewGameError::from)?;

        Ok(())
    }

    async fn add_player(
        &self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<()>, GameAssignmentError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(GameAssignmentError::from)?;
        let room = self
            .room_repository
            .get(room_id)
            .await
            .map_err(GameAssignmentError::from)?;

        if !Self::user_is_in_room(&user, &room) {
            return Err(UserNotInRoomError::new(user_id, room_id).into());
        }

        let game_id = self
            .room_repository
            .get(room_id)
            .await?
            .active_game_id
            .ok_or(GameAssignmentError::NoActiveGameInRoom(room_id))?;

        let mut game = self.game_repository.get(game_id).await?;
        if let Some(()) = game
            .add_player(user_id)
            .map_err(GameAssignmentError::from)?
        {
            self.game_repository.update(&game).await?;
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
}
