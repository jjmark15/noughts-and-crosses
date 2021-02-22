use std::sync::Arc;

use warp::Filter;

use crate::ports::http::warp::WsUserClientProviderAdapter;

pub(crate) fn with_user_client_provider(
    user_client_provider: Arc<WsUserClientProviderAdapter>,
) -> impl Filter<Extract = (Arc<WsUserClientProviderAdapter>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || user_client_provider.clone())
}
