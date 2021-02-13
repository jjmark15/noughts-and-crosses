use warp::reply::Response;
use warp::{Filter, Reply};

pub(crate) fn app_status_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().map(app_status_handler)
}

fn app_status_handler() -> StatusResponse {
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
