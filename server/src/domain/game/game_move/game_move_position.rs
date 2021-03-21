#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct GameMovePosition {
    x: u8,
    y: u8,
}

impl GameMovePosition {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        GameMovePosition { x, y }
    }

    pub(crate) fn x(&self) -> u8 {
        self.x
    }

    pub(crate) fn y(&self) -> u8 {
        self.y
    }
}
