use warp::Filter;

use crate::ports::http::warp::status_handler;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let status = warp::get()
            .and(warp::path!("admin" / "status"))
            .map(status_handler);

        warp::serve(status).run(([0, 0, 0, 0], 3030)).await;
    }
}
