use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct RegisteredUserResponse {
    id: Uuid,
}

impl RegisteredUserResponse {
    pub fn new(id: Uuid) -> Self {
        RegisteredUserResponse { id }
    }
}
