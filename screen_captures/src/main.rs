pub mod ipc_server;
use crate::ipc_server::socket_config;

/// Main server entry point
fn main() {
    let socket_config = socket_config::SocketConfig::default();
    ipc_server::ipc::start_ipc(&socket_config);
}
