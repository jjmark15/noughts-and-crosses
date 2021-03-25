use std::collections::HashSet;

use uuid::Uuid;

use crate::domain::game::{
    Game, GameMove, GamePlayService, GameRepository, StoreGameError, UserNotAPlayerInGameError,
};
use crate::domain::room::{AddPlayerError, GameMoveError, NewGameError, RemovePlayerError};

#[async_trait::async_trait]
pub(crate) trait GameManager {
    async fn start_new_game(&self) -> Result<Game, NewGameError>;

    async fn make_game_move(
        &self,
        user_id: Uuid,
        game_id: Uuid,
        game_move: GameMove,
    ) -> Result<(), GameMoveError>;

    async fn add_player(&self, game_id: Uuid, user_id: Uuid) -> Result<Option<()>, AddPlayerError>;

    async fn remove_player(&self, user_id: Uuid, game_id: Uuid) -> Result<(), RemovePlayerError>;
}

pub(crate) struct GameManagerImpl<GR: GameRepository, GPS: GamePlayService> {
    game_repository: GR,
    game_play_service: GPS,
}

impl<GR, GPS> GameManagerImpl<GR, GPS>
where
    GR: GameRepository,
    GPS: GamePlayService,
{
    pub(crate) fn new(game_repository: GR, game_play_service: GPS) -> Self {
        GameManagerImpl {
            game_repository,
            game_play_service,
        }
    }

    fn user_is_player(user_id: Uuid, game: &Game) -> bool {
        game.players().contains(&user_id)
    }
}

#[async_trait::async_trait]
impl<GR, GPS> GameManager for GameManagerImpl<GR, GPS>
where
    GR: GameRepository + Send + Sync,
    GPS: GamePlayService + Send + Sync,
{
    async fn start_new_game(&self) -> Result<Game, NewGameError> {
        let game = Game::new(Uuid::new_v4(), HashSet::new(), vec![]);
        match self.game_repository.store(&game).await {
            Ok(_) => Ok(game),
            Err(err) => match err {
                StoreGameError::AlreadyExists(_) => unimplemented!(),
            },
        }
    }

    async fn make_game_move(
        &self,
        user_id: Uuid,
        game_id: Uuid,
        game_move: GameMove,
    ) -> Result<(), GameMoveError> {
        let mut game = self.game_repository.get(game_id).await?;

        if !Self::user_is_player(user_id, &game) {
            return Err(UserNotAPlayerInGameError(user_id).into());
        }

        self.game_play_service
            .apply_move(&mut game, game_move, user_id)?;
        self.game_repository.update(&game).await?;
        Ok(())
    }

    async fn add_player(&self, game_id: Uuid, user_id: Uuid) -> Result<Option<()>, AddPlayerError> {
        let mut game = self.game_repository.get(game_id).await?;
        if let Some(()) = game.add_player(user_id).map_err(AddPlayerError::from)? {
            self.game_repository.update(&game).await?;
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    async fn remove_player(&self, user_id: Uuid, game_id: Uuid) -> Result<(), RemovePlayerError> {
        let mut game = self.game_repository.get(game_id).await?;
        game.remove_player(user_id);
        self.game_repository.update(&game).await?;
        Ok(())
    }
}
