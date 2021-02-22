use uuid::Uuid;

use crate::domain::user::User;

pub(crate) trait UserFactory {
    fn create<S: AsRef<str>>(&self, user_name: S) -> User;
}

pub(crate) struct UserFactoryImpl;

impl UserFactoryImpl {
    pub(crate) fn new() -> Self {
        UserFactoryImpl
    }
}

impl UserFactory for UserFactoryImpl {
    fn create<S: AsRef<str>>(&self, user_name: S) -> User {
        User::new(Uuid::new_v4(), user_name.as_ref().to_string(), None)
    }
}
