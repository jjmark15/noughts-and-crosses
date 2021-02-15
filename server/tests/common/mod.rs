use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::task::JoinHandle;

use functional_testing::http::{Client, Method};
use server::App;

pub(crate) async fn server_handle() -> &'static JoinHandle<()> {
    let handle = SERVER_HANDLE.handle();

    for _i in 0..20 {
        let response = Client::new()
            .request(Method::GET, "http://localhost:3030/admin/status")
            .send()
            .await;

        if response.is_err() {
            sleep(Duration::from_millis(250));
        }
    }

    handle
}

struct ServerHandle {
    handle: JoinHandle<()>,
}

impl ServerHandle {
    fn new() -> Self {
        ServerHandle {
            handle: tokio::task::spawn(async { App::new().run().await }),
        }
    }

    fn handle(&self) -> &JoinHandle<()> {
        &self.handle
    }
}

lazy_static! {
    static ref SERVER_HANDLE: ServerHandle = ServerHandle::new();
}
