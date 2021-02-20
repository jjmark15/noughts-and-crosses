use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;

pub(crate) fn with_application_service<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (Arc<AS>,), Error = std::convert::Infallible> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::any().map(move || application_service.clone())
}
