use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, NewGameError};
use crate::domain::room::NewGameError as DomainNewGameError;
use crate::ports::http::warp::responses::SimpleErrorResponse;
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn start_new_game_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync + 'static,
{
    warp::post()
        .and(warp::header("user-id"))
        .and(warp::header("room-id"))
        .and(with_application_service(application_service))
        .and_then(handler)
}

async fn handler<AS>(
    user_id: Uuid,
    room_id: Uuid,
    application_service: Arc<AS>,
) -> Result<impl Reply, Infallible>
where
    AS: ApplicationService + Send + Sync + 'static,
{
    let response = match application_service.start_new_game(room_id, user_id).await {
        Ok(_) => warp::reply::with_status(warp::reply(), StatusCode::CREATED).into_response(),
        Err(err) => new_game_error_response(err),
    };

    Ok(response)
}

fn new_game_error_response(err: NewGameError) -> Response {
    let status_code: StatusCode = match err.cause() {
        DomainNewGameError::UserNotFound(_) | DomainNewGameError::RoomNotFound(_) => {
            StatusCode::NOT_FOUND
        }
        DomainNewGameError::UserNotInRoom(_) => StatusCode::NOT_ACCEPTABLE,
    };
    let error_body = SimpleErrorResponse::new(err.to_string());
    json_reply_with_status(&error_body, status_code).into_response()
}
