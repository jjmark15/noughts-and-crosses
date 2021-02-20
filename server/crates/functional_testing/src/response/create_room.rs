use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct CreatedRoomResponse {
    id: Uuid,
}

impl CreatedRoomResponse {
    pub fn new(id: Uuid) -> Self {
        CreatedRoomResponse { id }
    }
}
