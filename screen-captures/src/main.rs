pub mod ipc_server;
pub mod screenshot;
pub mod ui;
use crate::ipc_server::socket_config;
use crate::screenshot::take_screenshot;
use crate::ui::events;
//use crate::ui::ui_screenshot;
use tokio;

/// Main server entry point
#[tokio::main]
async fn main() {
    let event= ui::ui_screenshot::ui_test();

    match event {
        events::Events::Screenshot | events::Events::Cancelled => {
            let respuesta_prto = take_screenshot::take_screenshot_proto().await;
            match respuesta_prto {
                Some(uri) => {
                    println!("Screenshot path: {}", uri);
                }

                None => {
                    println!("pinche gobierno puto. Se cancelo el proceso de la screenshot");
                }
            }
        }

        events::Events::ScreenshotOcr => {
            let respuesta_prto = take_screenshot::take_screenshot_proto().await;
            match respuesta_prto {
                Some(uri) => {
                    let socket_config = socket_config::SocketConfig::default();
                    ipc_server::ipc::start_ipc(&socket_config, &uri);
                }

                None => {
                    println!("pinche gobierno puto. Se cancelo el proceso de la screenshot");
                }
            }            
        }
    }
}
