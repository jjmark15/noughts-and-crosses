use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::ApplicationService;
use crate::domain::room::GameMoveError;
use crate::ports::http::warp::requests::{application_game_move, GameMove, GameMoveRequestError};
use crate::ports::http::warp::responses::SimpleErrorResponse;
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn make_game_move<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync + 'static,
{
    warp::post()
        .and(warp::header("user-id"))
        .and(warp::header("room-id"))
        .and(json_body())
        .and(with_application_service(application_service))
        .and_then(handler)
}

async fn handler<AS>(
    user_id: Uuid,
    room_id: Uuid,
    game_move: GameMove,
    application_service: Arc<AS>,
) -> Result<impl Reply, Infallible>
where
    AS: ApplicationService + Send + Sync + 'static,
{
    let response = match application_game_move(user_id, game_move) {
        Ok(game_move) => match application_service.make_game_move(room_id, game_move).await {
            Ok(_) => warp::reply::with_status(warp::reply(), StatusCode::ACCEPTED).into_response(),
            Err(err) => make_game_move_error_response(err),
        },
        Err(err) => game_move_request_error_response(err),
    };

    Ok(response)
}

fn game_move_request_error_response(err: GameMoveRequestError) -> Response {
    let status_code = StatusCode::NOT_ACCEPTABLE;

    let error_body = SimpleErrorResponse::new(err.to_string());
    json_reply_with_status(&error_body, status_code).into_response()
}

fn make_game_move_error_response(err: crate::application::GameMoveError) -> Response {
    let status_code = match &err.cause() {
        GameMoveError::NoActiveGameInRoom(_)
        | GameMoveError::UserNotFound(_)
        | GameMoveError::RoomNotFound(_) => StatusCode::NOT_FOUND,
        GameMoveError::PlayerCountExceeded(_)
        | GameMoveError::UserNotInRoom(_)
        | GameMoveError::UserNotPlayer(_)
        | GameMoveError::GamePlayError(_) => StatusCode::NOT_ACCEPTABLE,
        GameMoveError::GameNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let error_body = SimpleErrorResponse::new(err.to_string());
    json_reply_with_status(&error_body, status_code).into_response()
}

fn json_body() -> impl Filter<Extract = (GameMove,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
