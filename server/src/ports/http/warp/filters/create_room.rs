use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, RoomPersistenceError};
use crate::ports::http::warp::{json_reply_with_status, with_application_service};

pub(crate) fn create_room_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::post()
        .and(with_application_service(application_service))
        .and_then(create_room_handler)
}

async fn create_room_handler<AS: ApplicationService>(
    application_service: Arc<AS>,
) -> Result<CreateRoomResponse, Infallible> {
    let result = application_service.create_game_room().await;
    match result {
        Ok(id) => Ok(CreateRoomResponse::Success(id)),
        Err(err) => Ok(CreateRoomResponse::Error(err)),
    }
}

#[derive(Debug)]
enum CreateRoomResponse {
    Success(Uuid),
    Error(RoomPersistenceError),
}

#[derive(Debug, serde::Serialize)]
struct CreatedRoomResponse {
    id: Uuid,
}

impl CreatedRoomResponse {
    fn new(id: Uuid) -> Self {
        CreatedRoomResponse { id }
    }
}

impl Reply for CreateRoomResponse {
    fn into_response(self) -> Response {
        match self {
            CreateRoomResponse::Success(id) => {
                json_reply_with_status(&CreatedRoomResponse::new(id), StatusCode::CREATED)
            }
            CreateRoomResponse::Error(err) => match err.cause() {},
        }
    }
}
