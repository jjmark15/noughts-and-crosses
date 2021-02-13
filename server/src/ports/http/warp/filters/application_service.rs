use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;
use crate::domain::room::{RoomFactory, RoomRepository};

pub(crate) fn with_application_service<RR, RF>(
    application_service: Arc<ApplicationService<RR, RF>>,
) -> impl Filter<Extract = (Arc<ApplicationService<RR, RF>>,), Error = std::convert::Infallible> + Clone
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
{
    warp::any().map(move || application_service.clone())
}
