use server::App;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = App::new();

    app.run().await;
}
