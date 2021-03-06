use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, BecomePlayerError};
use crate::domain::room::AddPlayerError;
use crate::ports::http::warp::responses::SimpleErrorResponse;
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn become_player_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync + 'static,
{
    warp::put()
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
    let response = match application_service.become_player(room_id, user_id).await {
        Ok(Some(_)) => {
            warp::reply::with_status(warp::reply(), StatusCode::ACCEPTED).into_response()
        }
        Ok(None) => {
            warp::reply::with_status(warp::reply(), StatusCode::NOT_MODIFIED).into_response()
        }
        Err(err) => become_player_error_response(err),
    };

    Ok(response)
}

fn become_player_error_response(err: BecomePlayerError) -> Response {
    let status_code = match err.cause() {
        AddPlayerError::NoActiveGameInRoom(_)
        | AddPlayerError::UserNotFound(_)
        | AddPlayerError::RoomNotFound(_) => StatusCode::NOT_FOUND,
        AddPlayerError::PlayerCountExceeded(_) => StatusCode::NOT_ACCEPTABLE,
        AddPlayerError::UserNotInRoom(_) => StatusCode::NOT_ACCEPTABLE,
        AddPlayerError::GameNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let error_body = SimpleErrorResponse::new(err.to_string());
    json_reply_with_status(&error_body, status_code).into_response()
}
