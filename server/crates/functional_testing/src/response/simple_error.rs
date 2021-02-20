#[derive(Debug, serde::Deserialize)]
pub struct SimpleErrorResponse {
    cause: String,
}

impl SimpleErrorResponse {
    pub fn cause(&self) -> &String {
        &self.cause
    }
}
