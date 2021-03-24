use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::Mutex;
use uuid::Uuid;

use crate::domain::room::{Room, RoomNotFoundError, RoomPersistenceError, RoomRepository};
use crate::domain::user::User;

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
            .ok_or(RoomPersistenceError::NotFound(RoomNotFoundError(room_id)))?;
        map.insert(room_id, room.into());
        Ok(())
    }

    async fn get(&self, room_id: Uuid) -> Result<Room, RoomPersistenceError> {
        let map = self.inner.lock();
        let stored_room = map
            .get(&room_id)
            .ok_or_else::<RoomPersistenceError, _>(|| RoomNotFoundError(room_id).into())?;
        Ok(from_stored_room(room_id, stored_room))
    }

    async fn have_member(&self, user: &User) -> Result<Vec<Room>, RoomPersistenceError> {
        let map = self.inner.lock();
        let rooms: Vec<Room> = map
            .iter()
            .filter(|(_id, stored_room)| stored_room.members.contains(&user.id()))
            .map(|(id, stored_room)| from_stored_room(*id, stored_room))
            .collect();
        Ok(rooms)
    }
}

struct StoredRoom {
    active_game_id: Option<Uuid>,
    members: HashSet<Uuid>,
}

impl StoredRoom {
    fn new(active_game_id: Option<Uuid>, members: HashSet<Uuid>) -> Self {
        StoredRoom {
            active_game_id,
            members,
        }
    }
}

impl From<&Room> for StoredRoom {
    fn from(room: &Room) -> Self {
        StoredRoom::new(room.active_game_id(), room.members().clone())
    }
}

fn from_stored_room(id: Uuid, stored_room: &StoredRoom) -> Room {
    Room::new(id, stored_room.active_game_id, stored_room.members.clone())
}
