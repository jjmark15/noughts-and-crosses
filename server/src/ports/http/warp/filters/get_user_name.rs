use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::application::ApplicationService;
use crate::domain::user::GetUserError;
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
) -> Result<Response, Infallible> {
    let result = application_service.get_user_name(user_id).await;
    match result {
        Ok(name) => {
            Ok(warp::reply::with_status(name.into_response(), StatusCode::FOUND).into_response())
        }
        Err(err) => Ok(get_user_name_error(err)),
    }
}

fn get_user_name_error(err: GetUserError) -> Response {
    match err {
        GetUserError::NotFound(_id) => json_reply_with_status(
            &SimpleErrorResponse::new(err.to_string()),
            StatusCode::NOT_FOUND,
        ),
    }
}
