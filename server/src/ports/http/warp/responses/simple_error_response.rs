#[derive(Debug, serde::Serialize)]
pub(crate) struct SimpleErrorResponse {
    cause: String,
}

impl SimpleErrorResponse {
    pub(crate) fn new(cause: String) -> Self {
        SimpleErrorResponse { cause }
    }
}
