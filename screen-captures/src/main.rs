pub mod ipc_server;
pub mod screenshot;
//use crate::ipc_server::socket_config;
use crate::screenshot::take_screenshot;
use tokio;

/// Main server entry point
#[tokio::main]
async fn main() {
    //let socket_config = socket_config::SocketConfig::default();
    //ipc_server::ipc::start_ipc(&socket_config);
    take_screenshot::take_screenshot_proto().await;
}
