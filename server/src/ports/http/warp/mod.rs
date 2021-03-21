use warp::http::StatusCode;
use warp::reply::Response;
use warp::Reply;

pub(crate) use filters::*;
pub(crate) use user_client::*;

mod filters;
mod requests;
mod responses;
mod user_client;

pub(crate) fn json_reply_with_status<T: serde::Serialize>(
    body: &T,
    status_code: StatusCode,
) -> Response {
    warp::reply::with_status(warp::reply::json(body), status_code).into_response()
}
