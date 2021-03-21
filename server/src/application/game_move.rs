use uuid::Uuid;

use crate::domain::game::{GameMove, GameMovePosition};

pub(crate) struct ApplicationServiceGameMove {
    position: ApplicationServiceGameMovePosition,
    user_id: Uuid,
}

impl ApplicationServiceGameMove {
    pub(crate) fn new(user_id: Uuid, x: u8, y: u8) -> Self {
        let position = ApplicationServiceGameMovePosition { x, y };
        ApplicationServiceGameMove { user_id, position }
    }
}

struct ApplicationServiceGameMovePosition {
    x: u8,
    y: u8,
}

impl From<ApplicationServiceGameMove> for GameMove {
    fn from(game_move: ApplicationServiceGameMove) -> Self {
        GameMove::new(
            game_move.user_id,
            GameMovePosition::new(game_move.position.x, game_move.position.y),
        )
    }
}
