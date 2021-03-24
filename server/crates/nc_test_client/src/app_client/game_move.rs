#[derive(Debug, serde::Serialize, Copy, Clone)]
pub struct GameMove {
    position: GameMovePosition,
}

impl GameMove {
    pub fn new(position: GameMovePosition) -> Self {
        GameMove { position }
    }
}

#[derive(Debug, serde::Serialize, Copy, Clone)]
pub struct GameMovePosition {
    x: i8,
    y: i8,
}

impl GameMovePosition {
    pub fn new(x: i8, y: i8) -> Self {
        GameMovePosition { x, y }
    }
}
