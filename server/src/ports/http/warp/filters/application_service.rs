use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;
use crate::domain::room::{RoomFactory, RoomRepository};
use crate::domain::user::{UserFactory, UserRepository};

pub(crate) fn with_application_service<RR, RF, UR, UF>(
    application_service: Arc<ApplicationService<RR, RF, UR, UF>>,
) -> impl Filter<
    Extract = (Arc<ApplicationService<RR, RF, UR, UF>>,),
    Error = std::convert::Infallible,
> + Clone
where
    RR: RoomRepository + Send + Sync,
    RF: RoomFactory + Send + Sync,
    UR: UserRepository + Send + Sync,
    UF: UserFactory + Send + Sync,
{
    warp::any().map(move || application_service.clone())
}
