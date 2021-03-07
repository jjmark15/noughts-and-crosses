use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use uuid::Uuid;

use crate::domain::room::{Room, RoomPersistenceError, RoomRepository};

type EmbeddedDb = Arc<Mutex<HashMap<Uuid, StoredRoom>>>;

pub(crate) struct MapRoomRepositoryAdapter {
    inner: EmbeddedDb,
}

impl MapRoomRepositoryAdapter {
    pub(crate) fn new() -> Self {
        MapRoomRepositoryAdapter {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl RoomRepository for MapRoomRepositoryAdapter {
    async fn store(&self, room: &Room) -> Result<(), RoomPersistenceError> {
        let mut map = self.inner.lock();
        map.insert(room.id(), room.into());
        Ok(())
    }

    async fn update(&self, room: &Room) -> Result<(), RoomPersistenceError> {
        let room_id = room.id();
        let mut map = self.inner.lock();
        let _stored_room = map
            .get(&room_id)
            .ok_or(RoomPersistenceError::RoomNotFound(room_id))?;
        map.insert(room_id, room.into());
        Ok(())
    }

    async fn get(&self, room_id: Uuid) -> Result<Room, RoomPersistenceError> {
        let map = self.inner.lock();
        let stored_room = map
            .get(&room_id)
            .ok_or(RoomPersistenceError::RoomNotFound(room_id))?;
        Ok(Room::new(room_id, stored_room.active_game_id))
    }
}

struct StoredRoom {
    active_game_id: Option<Uuid>,
}

impl StoredRoom {
    fn new(active_game_id: Option<Uuid>) -> Self {
        StoredRoom { active_game_id }
    }
}

impl From<&Room> for StoredRoom {
    fn from(room: &Room) -> Self {
        StoredRoom::new(room.active_game_id())
    }
}
