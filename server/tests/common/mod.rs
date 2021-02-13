use tokio::task::JoinHandle;

use server::App;

pub fn server_handle() -> JoinHandle<()> {
    tokio::task::spawn(async { App::new().run().await })
}
