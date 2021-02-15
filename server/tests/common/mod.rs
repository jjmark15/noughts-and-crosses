use lazy_static::lazy_static;
use tokio::task::JoinHandle;

use server::App;

pub(crate) fn server_handle() -> &'static JoinHandle<()> {
    SERVER_HANDLE.handle()
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
