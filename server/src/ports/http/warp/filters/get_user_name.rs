use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, UserPersistenceError};
use crate::domain::user::UserPersistenceError as DomainUserPersistenceError;
use crate::ports::http::warp::responses::SimpleErrorResponse;
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn get_user_name_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::get()
        .and(with_application_service(application_service))
        .and(warp::path!(Uuid))
        .and_then(get_user_name_handler)
}

async fn get_user_name_handler<AS: ApplicationService>(
    application_service: Arc<AS>,
    user_id: Uuid,
) -> Result<GetUserNameResponse, Infallible> {
    let result = application_service.get_user_name(user_id).await;
    match result {
        Ok(name) => Ok(GetUserNameResponse::Success(name)),
        Err(err) => Ok(GetUserNameResponse::Error(err)),
    }
}

#[derive(Debug)]
enum GetUserNameResponse {
    Success(String),
    Error(UserPersistenceError),
}

impl Reply for GetUserNameResponse {
    fn into_response(self) -> Response {
        match self {
            GetUserNameResponse::Success(id) => {
                warp::reply::with_status(id.into_response(), StatusCode::FOUND).into_response()
            }
            GetUserNameResponse::Error(err) => match err.cause() {
                DomainUserPersistenceError::UserNotFound(_id) => json_reply_with_status(
                    &SimpleErrorResponse::new(err.to_string()),
                    StatusCode::NOT_FOUND,
                ),
            },
        }
    }
}
