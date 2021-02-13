use warp::Filter;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let status = warp::get().and(warp::path!("admin" / "status")).map(|| {
            let status = StatusResponse::new("OK".to_string());
            warp::reply::json(&status)
        });

        warp::serve(status).run(([0, 0, 0, 0], 3030)).await;
    }
}

#[derive(Debug, serde::Serialize)]
struct StatusResponse {
    status: String,
}

impl StatusResponse {
    fn new(status: String) -> Self {
        StatusResponse { status }
    }
}
