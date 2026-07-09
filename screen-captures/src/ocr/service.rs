use crate::ocr::{OcrRequest, domain::result::OcrResult};
use crate::ocr::transport::socketpair::start_ipc_socketpair;

pub fn service(req: &OcrRequest) -> OcrResult {
    let dto_response = start_ipc_socketpair(req).unwrap();
    OcrResult::from(&dto_response)
}