use uuid::Uuid;

use crate::domain::game::{Game, GameMove, GameMovePosition};

pub(crate) trait GamePlayService {
    fn apply_move(&self, game: &mut Game, game_move: GameMove) -> Result<(), GamePlayServiceError>;
}

pub(crate) struct GamePlayServiceImpl;

impl GamePlayServiceImpl {
    pub(crate) fn new() -> Self {
        GamePlayServiceImpl
    }

    fn user_is_player(user_id: Uuid, game: &Game) -> bool {
        game.players().contains(&user_id)
    }

    fn occupied_positions(game: &Game) -> Vec<GameMovePosition> {
        game.moves.iter().map(GameMove::position).collect()
    }

    fn position_is_occupied(game: &Game, position: &GameMovePosition) -> bool {
        Self::occupied_positions(game).contains(position)
    }

    fn position_is_out_of_bounds(position: &GameMovePosition) -> bool {
        position.y() > 2 || position.x() > 2
    }
}

impl GamePlayService for GamePlayServiceImpl {
    fn apply_move(&self, game: &mut Game, game_move: GameMove) -> Result<(), GamePlayServiceError> {
        let user_id = game_move.user_id();
        if !Self::user_is_player(user_id, &game) {
            return Err(GamePlayServiceError::UserNotAPlayer(user_id));
        } else if Self::position_is_out_of_bounds(&game_move.position()) {
            return Err(GamePlayServiceError::PositionOutOfBounds);
        } else if Self::position_is_occupied(&game, &game_move.position()) {
            return Err(GamePlayServiceError::PositionIsAlreadyOccupied);
        }
        game.append_move(game_move);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GamePlayServiceError {
    #[error("User({0}) is not a player in game")]
    UserNotAPlayer(Uuid),
    #[error("Position is already occupied")]
    PositionIsAlreadyOccupied,
    #[error("Position is out of bounds")]
    PositionOutOfBounds,
}
