use warp::reply::Response;
use warp::Reply;

pub(crate) fn status_handler() -> StatusResponse {
    StatusResponse::new("OK".to_string())
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct StatusResponse {
    status: String,
}

impl StatusResponse {
    fn new(status: String) -> Self {
        StatusResponse { status }
    }
}

impl Reply for StatusResponse {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
