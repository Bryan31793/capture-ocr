use ashpd::desktop::screenshot::Screenshot;

pub async fn take_screenshot_proto() {
    let screenshot = Screenshot::request()
        .interactive(true)
        .send().await;
    
    let uri_screenshot = match screenshot {
        Ok(request) => {
            match request.response() {
                Ok(result) => {
                    result
                }
                Err(e) => {
                    println!("Error en la screenshot: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            println!("Error en la request de la screenshot: {}", e);
            return;
        }
    };
    
    let uri = uri_screenshot.uri();
    println!("URI: {}", uri.as_str());
}



