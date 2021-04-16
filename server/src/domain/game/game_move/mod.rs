use uuid::Uuid;

pub(crate) use error::*;
pub(crate) use game_move_position::*;

mod error;
mod game_move_position;

#[derive(Debug, Copy, Clone)]
pub(crate) struct GameMove {
    user_id: Uuid,
    position: GameMovePosition,
}

impl GameMove {
    pub(crate) fn new(user_id: Uuid, position: GameMovePosition) -> Self {
        GameMove { user_id, position }
    }

    pub(crate) fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub(crate) fn position(&self) -> GameMovePosition {
        self.position
    }
}
