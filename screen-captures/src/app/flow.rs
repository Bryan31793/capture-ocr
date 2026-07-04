use crate::ocr::service::service;
use crate::ocr::OcrRequest;
use crate::capture::screenshot::take_screenshot_proto;
use crate::ui::events::Events;
use serde_json::json;
use std::thread;
use std::time::Duration;

pub async fn run() {
    let event = crate::ui::screenshot_ocr_window::ui_test();

    thread::sleep(Duration::from_millis(100));
    match event {
        Events::Screenshot | Events::Cancelled => {
            let screenshot_path = take_screenshot_proto().await;
            match screenshot_path {
                Some(uri) => {
                    println!("Screenshot path: {}", uri);
                }
                None => {
                    println!("pinche gobierno puto. Se cancelo el proceso de la screenshot");
                }
            }
        }
        Events::ScreenshotOcr => {
            let screenshot_path = take_screenshot_proto().await;
            match screenshot_path {
                Some(uri) => {
                    let request = OcrRequest {
                        action: String::from("extract text"),
                        payload: json!({
                            "path": uri
                        }),
                    };
                    let text = service(&request);
                    println!("Rust recibio: {}", text.text());
                    //start_ipc_socketpair(&request);
                }
                None => {
                    println!("pinche gobierno puto. Se cancelo el proceso de la screenshot");
                }
            }
        }
    }
}