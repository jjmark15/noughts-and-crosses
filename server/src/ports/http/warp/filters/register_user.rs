use std::convert::Infallible;
use std::sync::Arc;

use warp::http::StatusCode;
use warp::reply::Response;
use warp::Filter;

use crate::application::{ApplicationService, RegisterUserError};
use crate::domain::user::StoreUserError;
use crate::ports::http::warp::responses::RegisterUserResponse;
use crate::ports::http::warp::{json_reply_with_status, with_application_service, PercentDecoded};

pub(crate) fn register_user_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::post()
        .and(with_application_service(application_service))
        .and(warp::path!(PercentDecoded))
        .and_then(register_user_handler)
}

async fn register_user_handler<AS: ApplicationService>(
    application_service: Arc<AS>,
    decoded_user_name: PercentDecoded,
) -> Result<Response, Infallible> {
    let result = application_service
        .register_user(decoded_user_name.to_string())
        .await;
    match result {
        Ok(id) => Ok(json_reply_with_status(
            &RegisterUserResponse::new(id),
            StatusCode::CREATED,
        )),
        Err(err) => Ok(register_user_error_response(err)),
    }
}

fn register_user_error_response(err: RegisterUserError) -> Response {
    match err.cause() {
        StoreUserError::AlreadyExists(_) => unimplemented!(),
    }
}
