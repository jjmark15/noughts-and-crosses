use uuid::Uuid;

use crate::domain::user::UserClient;

#[async_trait::async_trait]
pub(crate) trait UserClientProvider {
    type UserClient: UserClient;

    async fn get(&self, user_id: Uuid) -> Result<Self::UserClient, UserClientProviderError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum UserClientProviderError {
    #[error("Could not find client for specified user")]
    UserClientNotAvailable,
}
