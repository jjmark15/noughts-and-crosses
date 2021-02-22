use std::convert::Infallible;
use std::sync::Arc;

use futures_util::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::ws::WebSocket;
use warp::{Filter, Reply};

use crate::application::{ApplicationService, JoinRoomError, RoomCreationError};
use crate::domain::RoomAssignmentError;
use crate::ports::http::warp::{
    with_application_service, with_user_client_provider, WsUserClientAdapter,
    WsUserClientProviderAdapter,
};

pub(crate) fn join_new_room_filter<AS>(
    application_service: Arc<AS>,
    user_client_provider: Arc<WsUserClientProviderAdapter>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync + 'static,
{
    warp::any()
        .and(warp::header("user-id"))
        .and(warp::ws())
        .and(with_application_service(application_service))
        .and(with_user_client_provider(user_client_provider))
        .and_then(handler)
}

async fn handler<AS>(
    user_id: Uuid,
    ws: warp::ws::Ws,
    application_service: Arc<AS>,
    user_client_provider: Arc<WsUserClientProviderAdapter>,
) -> Result<impl Reply, Infallible>
where
    AS: ApplicationService + Send + Sync + 'static,
{
    let room_id = match application_service.create_room().await {
        Ok(room_id) => room_id,
        Err(err) => return Ok(create_room_error_response(err)),
    };

    let connected_application_service = application_service.clone();
    let reply = ws.on_upgrade(move |ws| {
        user_connected(
            ws,
            connected_application_service,
            user_client_provider,
            user_id,
        )
    });

    // room join should happen after socket upgrade to avoid trying to interact with user before client is ready
    if let Err(err) = join_room(user_id, application_service, room_id).await {
        return Ok(join_room_error_response(err));
    }

    Ok(warp::reply::with_header(reply, "room-id", room_id.to_string()).into_response())
}

async fn user_connected<AS>(
    ws: WebSocket,
    application_service: Arc<AS>,
    user_client_provider: Arc<WsUserClientProviderAdapter>,
    user_id: Uuid,
) where
    AS: ApplicationService + Send + Sync,
{
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::channel(10);
    let rx = ReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(user_ws_tx).map(move |result| {
        if let Err(err) = result {
            log::error!(
                "Error while sending WebSocket message to user({}): {}",
                user_id,
                err
            );
        }
    }));

    let user_client = WsUserClientAdapter::new(user_id, Arc::new(tx));
    user_client_provider.put(user_client).await;

    while let Some(result) = user_ws_rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_close() {
                    break;
                } else {
                    unimplemented!()
                }
            }
            Err(err) => {
                log::error!(
                    "WebSocket error while reading message from user({}): {}",
                    user_id,
                    err
                );
                break;
            }
        };
    }

    user_disconnected(application_service, user_id, user_client_provider).await;
}

async fn user_disconnected<AS>(
    application_service: Arc<AS>,
    user_id: Uuid,
    user_client_provider: Arc<WsUserClientProviderAdapter>,
) where
    AS: ApplicationService + Send + Sync,
{
    if let Err(err) = application_service.leave_room(user_id).await {
        log::error!("Error while disconnecting user({}): {}", user_id, err);
    }
    user_client_provider.remove(user_id).await;
}

fn join_room_error_response(err: JoinRoomError) -> Response {
    let status_code = match &err {
        JoinRoomError::RoomAssignment(assignment_error) => match assignment_error {
            RoomAssignmentError::AlreadyAssigned => StatusCode::CONFLICT,
            RoomAssignmentError::UserPersistence(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
    };
    warp::reply::with_status(warp::reply(), status_code).into_response()
}

fn create_room_error_response(_err: RoomCreationError) -> Response {
    warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

async fn join_room<AS>(
    user_id: Uuid,
    application_service: Arc<AS>,
    room_id: Uuid,
) -> Result<(), JoinRoomError>
where
    AS: ApplicationService + Send + Sync + 'static,
{
    if let Err(join_err) = application_service.join_room(room_id, user_id).await {
        log::error!("User({}) failed to join room: {}", user_id, join_err);
        if let Err(leave_err) = application_service.leave_room(user_id).await {
            log::error!("User({}) failed to leave room: {}", user_id, leave_err);
        }

        return Err(join_err);
    }

    Ok(())
}
