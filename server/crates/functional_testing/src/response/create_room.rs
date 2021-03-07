use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct CreateRoomResponse {
    room_id: String,
}

impl CreateRoomResponse {
    pub fn new(id: Uuid) -> Self {
        CreateRoomResponse {
            room_id: id.to_string(),
        }
    }

    pub fn room_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.room_id.as_str())
    }
}
