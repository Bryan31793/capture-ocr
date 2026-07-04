use crate::ocr::OcrResponse;

/// TODO:
/// add accuracy field
pub struct OcrResult {
    text: String,
}

impl OcrResult {
    //idk if &str is the best option
    pub fn text(&self) -> &str {
        &self.text
    }
}

//falta pulir este trait
//talvez deba usar TryFrom en vez de From
impl From<&OcrResponse> for OcrResult {
    fn from(value: &OcrResponse) -> Self {
        let data_text = match value {
            OcrResponse::Ok { data } => {
                data.as_str().unwrap().to_string()
            },
            OcrResponse::Error { .. } => {
                String::from("Error")
            }
        };

        OcrResult { text: data_text }
    }
}