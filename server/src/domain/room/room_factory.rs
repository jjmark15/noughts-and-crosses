use std::collections::HashSet;

use uuid::Uuid;

use crate::domain::room::Room;

pub(crate) trait RoomFactory {
    fn create(&self) -> Room;
}

pub(crate) struct RoomFactoryImpl;

impl RoomFactoryImpl {
    pub(crate) fn new() -> Self {
        RoomFactoryImpl
    }
}

impl RoomFactory for RoomFactoryImpl {
    fn create(&self) -> Room {
        Room::new(Uuid::new_v4(), None, HashSet::new())
    }
}
