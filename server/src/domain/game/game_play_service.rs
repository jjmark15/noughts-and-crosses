use uuid::Uuid;

use crate::domain::game::{
    AttemptedMoveOutOfTurnError, Game, GameMove, GameMovePosition, PositionIsAlreadyOccupiedError,
    PositionOutOfBoundsError,
};

pub(crate) trait GamePlayService {
    fn apply_move(
        &self,
        game: &mut Game,
        game_move: GameMove,
        user_id: Uuid,
    ) -> Result<(), ApplyMoveError>;
}

pub(crate) struct GamePlayServiceImpl;

impl GamePlayServiceImpl {
    pub(crate) fn new() -> Self {
        GamePlayServiceImpl
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

    fn is_players_turn(user_id: Uuid, game: &Game) -> bool {
        true
    }
}

impl GamePlayService for GamePlayServiceImpl {
    fn apply_move(
        &self,
        game: &mut Game,
        game_move: GameMove,
        user_id: Uuid,
    ) -> Result<(), ApplyMoveError> {
        if !Self::is_players_turn(user_id, &game) {
            return Err(AttemptedMoveOutOfTurnError.into());
        } else if Self::position_is_out_of_bounds(&game_move.position()) {
            return Err(PositionOutOfBoundsError.into());
        } else if Self::position_is_occupied(&game, &game_move.position()) {
            return Err(PositionIsAlreadyOccupiedError.into());
        }
        game.append_move(game_move);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ApplyMoveError {
    #[error("Position is already occupied")]
    PositionIsAlreadyOccupied(#[from] PositionIsAlreadyOccupiedError),
    #[error("Position is out of bounds")]
    PositionOutOfBounds(#[from] PositionOutOfBoundsError),
    #[error(transparent)]
    AttemptedMoveOutOfTurn(#[from] AttemptedMoveOutOfTurnError),
}
