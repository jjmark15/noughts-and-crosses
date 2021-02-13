#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct AppStatusResponse {
    status: String,
}

impl AppStatusResponse {
    pub fn new(status: String) -> Self {
        AppStatusResponse { status }
    }
}
