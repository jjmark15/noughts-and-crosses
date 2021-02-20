use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, UserPersistenceError};
use crate::domain::room::{RoomFactory, RoomRepository};
use crate::domain::user::{UserFactory, UserRepository};
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn register_user_filter<RR, RF, UR, UF>(
    application_service: Arc<ApplicationService<RR, RF, UR, UF>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
    UR: UserRepository + Send + Sync,
    UF: UserFactory + Send + Sync,
{
    warp::post()
        .and(with_application_service(application_service))
        .and(warp::path!(String))
        .and_then(register_user_handler)
}

async fn register_user_handler<
    RR: RoomRepository,
    RF: RoomFactory,
    UR: UserRepository,
    UF: UserFactory,
>(
    application_service: Arc<ApplicationService<RR, RF, UR, UF>>,
    user_name: String,
) -> Result<RegisterUserResponse, Infallible> {
    let result = application_service.register_user(user_name).await;
    match result {
        Ok(id) => Ok(RegisterUserResponse::Success(id)),
        Err(err) => Ok(RegisterUserResponse::Error(err)),
    }
}

#[derive(Debug)]
enum RegisterUserResponse {
    Success(Uuid),
    Error(UserPersistenceError),
}

#[derive(Debug, serde::Serialize)]
struct RegisteredUserResponse {
    id: Uuid,
}

impl RegisteredUserResponse {
    fn new(id: Uuid) -> Self {
        RegisteredUserResponse { id }
    }
}

impl Reply for RegisterUserResponse {
    fn into_response(self) -> Response {
        match self {
            RegisterUserResponse::Success(id) => {
                json_reply_with_status(&RegisteredUserResponse::new(id), StatusCode::CREATED)
            }
            RegisterUserResponse::Error(err) => match err.cause() {},
        }
    }
}
