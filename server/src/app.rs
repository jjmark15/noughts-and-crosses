use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;
use crate::domain::room::RoomFactoryImpl;
use crate::ports::http::warp::{app_status_filter, create_room_filter};
use crate::ports::persistence::vec::VecRoomRepositoryAdapter;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let room_repository = VecRoomRepositoryAdapter::new();
        let room_factory = RoomFactoryImpl::new();
        let application_service = ApplicationService::new(room_repository, room_factory);

        let routes = warp::any()
            .and(warp::path("admin").and(Self::admin_routes()))
            .or(warp::path("game").and(Self::game_routes(application_service)));

        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    }

    fn admin_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        let status = warp::path("status").and(app_status_filter());

        warp::any().and(status)
    }

    fn game_routes(
        application_service: ApplicationService<VecRoomRepositoryAdapter, RoomFactoryImpl>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let create_room = warp::path("room").and(create_room_filter(Arc::new(application_service)));

        warp::any().and(create_room)
    }
}
