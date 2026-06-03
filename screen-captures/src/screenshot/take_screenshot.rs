//use adw::gio::FileAttributeType::String;
//use adw::glib::VariantClass::String;
use ashpd::desktop::screenshot::Screenshot;

pub async fn take_screenshot_proto() -> Option<std::string::String> {
    let screenshot = Screenshot::request()
        .interactive(true)
        .send().await;
    
    let uri_screenshot = match screenshot {
        Ok(request) => {
            match request.response() {
                Ok(result) => {
                    result
                }
                Err(_e) => {
                    return None;
                }
            }
        }
        Err(e) => {
            println!("Error en la request de la screenshot: {}", e);
            return None;
        }
    };
    
    Some(
        std::string::String::from(
            uri_screenshot
            .uri()
            .as_str()
        )
    )
    //println!("URI: {}", uri.as_str());
}



