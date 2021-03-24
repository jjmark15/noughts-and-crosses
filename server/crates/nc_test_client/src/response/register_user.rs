use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct RegisteredUserResponse {
    #[serde(rename = "id")]
    user_id: String,
}

impl RegisteredUserResponse {
    pub fn new(user_id: Uuid) -> Self {
        RegisteredUserResponse {
            user_id: user_id.to_string(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.user_id)
    }
}
