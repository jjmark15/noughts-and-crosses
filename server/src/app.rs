use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationServiceImpl;
use crate::domain::room::{RoomFactoryImpl, RoomManagerImpl};
use crate::domain::user::UserFactoryImpl;
use crate::ports::http::warp::{
    app_status_filter, create_room_filter, get_user_name_filter, join_room_filter,
    register_user_filter, start_new_game_filter, WsUserClientProviderAdapter,
};
use crate::ports::persistence::map::{
    MapGameRepositoryAdapter, MapRoomRepositoryAdapter, MapUserRepositoryAdapter,
};

type ApplicationServiceAlias = ApplicationServiceImpl<
    MapRoomRepositoryAdapter,
    RoomFactoryImpl,
    MapUserRepositoryAdapter,
    UserFactoryImpl,
    RoomManagerImpl<MapUserRepositoryAdapter, MapRoomRepositoryAdapter, MapGameRepositoryAdapter>,
>;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let user_client_provider = Arc::new(WsUserClientProviderAdapter::new());
        let application_service = Self::application_service();

        let routes = warp::any()
            .and(warp::path("admin").and(Self::admin_routes()))
            .or(warp::path("game")
                .and(Self::game_routes(application_service, user_client_provider)));

        let socket_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 3030);
        warp::serve(routes).run(socket_address).await;
    }

    fn admin_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        let status = warp::path("status").and(app_status_filter());

        warp::any().and(status)
    }

    fn game_routes(
        application_service: ApplicationServiceAlias,
        user_client_provider: Arc<WsUserClientProviderAdapter>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let application_service = Arc::new(application_service);

        let create_room = create_room_filter(application_service.clone()).and(warp::path::end());
        let join_room = join_room_filter(application_service.clone(), user_client_provider)
            .and(warp::path::end());
        let rooms = warp::path("rooms").and(create_room.or(join_room));

        let start_new_game =
            start_new_game_filter(application_service.clone()).and(warp::path::end());
        let games = warp::path("games").and(start_new_game);

        let users = warp::path("users").and(
            register_user_filter(application_service.clone())
                .or(get_user_name_filter(application_service)),
        );

        warp::any().and(users).or(rooms).or(games)
    }

    fn application_service() -> ApplicationServiceAlias {
        let room_repository = Arc::new(MapRoomRepositoryAdapter::new());
        let room_factory = RoomFactoryImpl::new();
        let user_repository = Arc::new(MapUserRepositoryAdapter::new());
        let user_factory = UserFactoryImpl::new();
        let game_repository = MapGameRepositoryAdapter::new();
        let room_manager = RoomManagerImpl::new(
            user_repository.clone(),
            room_repository.clone(),
            game_repository,
        );
        ApplicationServiceImpl::new(
            room_repository,
            room_factory,
            user_repository,
            user_factory,
            room_manager,
        )
    }
}
