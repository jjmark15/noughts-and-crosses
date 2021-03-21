use functional_testing::{GameMove, GameMovePosition};

pub(crate) fn top_left() -> GameMove {
    GameMove::new(GameMovePosition::new(0, 0))
}

pub(crate) fn x_position_below_valid_range() -> GameMove {
    GameMove::new(GameMovePosition::new(-1, 0))
}

pub(crate) fn x_position_above_valid_range() -> GameMove {
    GameMove::new(GameMovePosition::new(3, 0))
}

pub(crate) fn y_position_below_valid_range() -> GameMove {
    GameMove::new(GameMovePosition::new(0, -1))
}

pub(crate) fn y_position_above_valid_range() -> GameMove {
    GameMove::new(GameMovePosition::new(0, 3))
}
