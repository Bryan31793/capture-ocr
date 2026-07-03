pub mod ipc_server;
pub mod screenshot;
pub mod ui;
use crate::screenshot::take_screenshot;
use crate::ui::events;
use serde_json::json;
//use crate::ui::ui_screenshot;
use tokio;
use std::thread;
use std::time::Duration;

/// Main server entry point
#[tokio::main]
async fn main() {

    
    let event= ui::ui_screenshot::ui_test();

    thread::sleep(Duration::from_millis(100));
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
                    let req = ipc_server::json_data::OcrRequest {
                        action: String::from("extract text"),
                        payload: json!({
                            "path": uri
                        }),
                    };
                    //dbg!(&req);
                    //let socket_config = socket_config::SocketConfig::default();
                    ipc_server::ipc_socketpair::start_ipc_socketpair(&req);
                }

                None => {
                    println!("pinche gobierno puto. Se cancelo el proceso de la screenshot");
                }
            }            
        }
    }
    /* 
   let request = OcrRequest{
    action: String::from("extract text"),
    payload: json!({
        "path": "/home/bryan/capture-ocr/ocr/captures/hello_world.png"
    })
   };
   ipc_socketpair::start_ipc_socketpair(&request);*/
}

