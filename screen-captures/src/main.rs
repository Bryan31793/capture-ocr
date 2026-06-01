pub mod ipc_server;
pub mod screenshot;
pub mod ui;
use crate::ipc_server::socket_config;
use crate::screenshot::take_screenshot;
//use crate::ui::ui_screenshot;
use tokio;

/// Main server entry point
#[tokio::main]
async fn main() {
    
    let respuesta_prto = take_screenshot::take_screenshot_proto().await;

    match respuesta_prto {
        Some(uri) => {
            let socket_config = socket_config::SocketConfig::default();
            ipc_server::ipc::start_ipc(&socket_config, &uri);
        }

        None => {
            println!("pinche gobierno puto");
        }
    }
    //ui::ui_screenshot::ui_test();
}
