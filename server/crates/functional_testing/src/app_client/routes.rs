use std::fmt::{Display, Formatter};

use uuid::Uuid;

pub(super) enum Route {
    Status,
    CreateRoom,
    JoinRoom(Uuid),
    StartNewGame,
    RegisterUser(String),
    UserName(String),
}

impl Route {
    pub(crate) fn url_path(&self) -> UrlPath {
        match self {
            Route::Status => UrlPath::new().with_segment("admin").with_segment("status"),
            Route::CreateRoom => UrlPath::new().with_segment("game").with_segment("rooms"),
            Route::JoinRoom(room_id) => UrlPath::new()
                .with_segment("game")
                .with_segment("rooms")
                .with_segment(room_id)
                .with_segment("members"),
            Route::StartNewGame => UrlPath::new().with_segment("game").with_segment("games"),
            Route::RegisterUser(user_name) => UrlPath::new()
                .with_segment("game")
                .with_segment("users")
                .with_segment(user_name),
            Route::UserName(user_id) => UrlPath::new()
                .with_segment("game")
                .with_segment("users")
                .with_segment(user_id),
        }
    }
}

pub(crate) struct UrlPath {
    segments: Vec<String>,
}

impl UrlPath {
    fn new() -> Self {
        UrlPath { segments: vec![] }
    }

    fn with_segment(mut self, segment: impl ToString) -> Self {
        self.segments.push(segment.to_string());
        self
    }
}

impl Display for UrlPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.segments.join("/"))
    }
}
