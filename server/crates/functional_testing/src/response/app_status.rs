#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub struct AppStatus {
    status: String,
}

impl AppStatus {
    pub fn new(status: String) -> Self {
        AppStatus { status }
    }
}
