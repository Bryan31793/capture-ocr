//use adw::gio::FileAttributeType::String;
//use adw::glib::VariantClass::String;
use ashpd::desktop::screenshot::Screenshot;
use urlencoding::decode;

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
        parse_uri(
            std::string::String::from(
            uri_screenshot
            .uri()
            .as_str()
            )
        )
    )
    //println!("URI: {}", uri.as_str());
}

/// Parse the uri from the screenshot. 
/// Eliminate unnecesary prefix and decode percent-encoded string
fn parse_uri(uri: String) -> String {
    let n = uri.len() - 7;
    let sub: String = uri.chars().skip(7).take(n).collect();
    let uri_decoded = decode(&sub).expect("UTF-8").into_owned();
    uri_decoded
}



