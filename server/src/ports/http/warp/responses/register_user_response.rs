use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub(crate) struct RegisterUserResponse {
    id: Uuid,
}

impl RegisterUserResponse {
    pub(crate) fn new(id: Uuid) -> Self {
        RegisterUserResponse { id }
    }
}
