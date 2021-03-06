use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub(crate) struct CreateRoomResponse {
    room_id: Uuid,
}

impl CreateRoomResponse {
    pub(crate) fn new(room_id: Uuid) -> Self {
        CreateRoomResponse { room_id }
    }
}
