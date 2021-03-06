use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, RoomCreationError};
use crate::domain::user::UserPersistenceError;
use crate::ports::http::warp::responses::{CreateRoomResponse, SimpleErrorResponse};
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn create_room_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync + 'static,
{
    warp::any()
        .and(warp::header("user-id"))
        .and(with_application_service(application_service))
        .and_then(handler)
}

async fn handler<AS>(user_id: Uuid, application_service: Arc<AS>) -> Result<impl Reply, Infallible>
where
    AS: ApplicationService + Send + Sync + 'static,
{
    if let Err(err) = application_service.get_user_name(user_id).await {
        return match err.cause() {
            UserPersistenceError::UserNotFound(_) => Ok(json_reply_with_status(
                &SimpleErrorResponse::new(err.to_string()),
                StatusCode::NOT_FOUND,
            )),
        };
    }

    let response = match application_service.create_room().await {
        Ok(room_id) => {
            json_reply_with_status(&CreateRoomResponse::new(room_id), StatusCode::CREATED)
        }
        Err(err) => create_room_error_response(err),
    };

    Ok(response)
}

fn create_room_error_response(_err: RoomCreationError) -> Response {
    warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
}
