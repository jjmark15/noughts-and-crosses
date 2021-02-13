use server::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    app.run().await;
}
