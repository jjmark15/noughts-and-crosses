use std::sync::Arc;

use uuid::Uuid;

pub(crate) use error::*;

use crate::domain::game::{GameManager, GameMove};
use crate::domain::room::{Room, RoomRepository};
use crate::domain::user::{User, UserRepository};

mod error;

#[async_trait::async_trait]
pub(crate) trait RoomManager {
    async fn join_room(&self, user_id: Uuid, room_id: Uuid) -> Result<(), JoinRoomError>;

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError>;

    async fn start_new_game(&self, room_id: Uuid, user_id: Uuid) -> Result<(), NewGameError>;

    async fn make_game_move(&self, room_id: Uuid, game_move: GameMove)
        -> Result<(), GameMoveError>;

    async fn add_player(&self, room_id: Uuid, user_id: Uuid) -> Result<Option<()>, AddPlayerError>;
}

pub(crate) struct RoomManagerImpl<UR: UserRepository, RR: RoomRepository, GM: GameManager> {
    user_repository: Arc<UR>,
    room_repository: Arc<RR>,
    game_manager: GM,
}

impl<UR, RR, GM> RoomManagerImpl<UR, RR, GM>
where
    UR: UserRepository,
    RR: RoomRepository,
    GM: GameManager,
{
    pub(crate) fn new(
        user_repository: Arc<UR>,
        room_repository: Arc<RR>,
        game_manager: GM,
    ) -> Self {
        RoomManagerImpl {
            user_repository,
            room_repository,
            game_manager,
        }
    }

    fn user_is_in_room(user: &User, room: &Room) -> bool {
        room.is_member(user.id())
    }
}

#[async_trait::async_trait]
impl<UR, RR, GM> RoomManager for RoomManagerImpl<UR, RR, GM>
where
    UR: UserRepository + Send + Sync,
    RR: RoomRepository + Send + Sync,
    GM: GameManager + Send + Sync,
{
    async fn join_room(&self, user_id: Uuid, room_id: Uuid) -> Result<(), JoinRoomError> {
        let user = self.user_repository.get(user_id).await?;
        if !self.room_repository.have_member(&user).await.is_empty() {
            return Err(JoinRoomError::AlreadyAssigned);
        }
        let mut room = self.room_repository.get(room_id).await?;
        room.add_member(user.id());
        self.room_repository.update(&room).await?;
        Ok(())
    }

    async fn leave_room(&self, user_id: Uuid) -> Result<(), LeaveRoomError> {
        let user = self.user_repository.get(user_id).await?;
        let mut rooms = self.room_repository.have_member(&user).await;
        rooms
            .iter_mut()
            .for_each(|room| room.remove_member(user_id));
        for room in &rooms {
            if let Some(game_id) = room.active_game_id() {
                self.game_manager.remove_player(user_id, game_id).await?;
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

        let game = self.game_manager.start_new_game().await?;
        room.set_active_game_id(game.id());
        self.room_repository
            .update(&room)
            .await
            .map_err(NewGameError::from)?;

        Ok(())
    }

    async fn make_game_move(
        &self,
        room_id: Uuid,
        game_move: GameMove,
    ) -> Result<(), GameMoveError> {
        let user = self
            .user_repository
            .get(game_move.user_id())
            .await
            .map_err(GameMoveError::from)?;
        let room = self.room_repository.get(room_id).await?;

        if !Self::user_is_in_room(&user, &room) {
            return Err(UserNotInRoomError::new(user.id(), room_id).into());
        }

        match room.active_game_id() {
            Some(game_id) => {
                self.game_manager
                    .make_game_move(user.id(), game_id, game_move)
                    .await
            }
            None => Err(NoActiveGameInRoomError(room_id).into()),
        }
    }

    async fn add_player(&self, room_id: Uuid, user_id: Uuid) -> Result<Option<()>, AddPlayerError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(AddPlayerError::from)?;
        let room = self
            .room_repository
            .get(room_id)
            .await
            .map_err(AddPlayerError::from)?;

        if !Self::user_is_in_room(&user, &room) {
            return Err(UserNotInRoomError::new(user_id, room_id).into());
        }

        let game_id = self
            .room_repository
            .get(room_id)
            .await?
            .active_game_id
            .ok_or(NoActiveGameInRoomError(room_id))?;

        self.game_manager.add_player(game_id, user_id).await
    }
}
