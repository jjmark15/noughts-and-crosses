use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

pub(crate) fn app_status_filter(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().map(app_status_handler)
}

fn app_status_handler() -> Response {
    warp::reply::with_status("OK", StatusCode::OK).into_response()
}

#[derive(Debug, serde::Serialize)]
struct StatusResponse {
    status: String,
}
