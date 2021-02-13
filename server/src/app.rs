use warp::Filter;

use crate::ports::http::warp::status_handler;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let routes = warp::any().and(warp::path("admin").and(Self::admin_routes()));

        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    }

    fn admin_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        let status = warp::get().and(warp::path("status")).map(status_handler);

        warp::any().and(status)
    }
}
