use uuid::Uuid;

use crate::application::ApplicationServiceGameMove;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GameMove {
    position: GameMovePosition,
}

#[derive(Debug, serde::Deserialize)]
struct GameMovePosition {
    x: i8,
    y: i8,
}

pub(crate) fn application_game_move(
    user_id: Uuid,
    game_move_request: GameMove,
) -> Result<ApplicationServiceGameMove, GameMoveRequestError> {
    Ok(ApplicationServiceGameMove::new(
        user_id,
        signed_to_unsigned_integer(game_move_request.position.x)
            .map_err(|_| GameMoveRequestError)?,
        signed_to_unsigned_integer(game_move_request.position.y)
            .map_err(|_| GameMoveRequestError)?,
    ))
}

fn signed_to_unsigned_integer(signed: i8) -> Result<u8, ()> {
    if signed >= 0 {
        Ok(signed as u8)
    } else {
        Err(())
    }
}

#[derive(Debug, thiserror::Error, Default)]
#[error("Game move request object is invalid")]
pub(crate) struct GameMoveRequestError;
