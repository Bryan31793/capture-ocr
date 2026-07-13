use crate::ocr::{OcrRequest, domain::result::OcrResult};
use crate::ocr::config::runtime_config::RuntimeConfig;
use crate::ocr::transport::socketpair::start_ipc_socketpair;

pub fn service(req: &OcrRequest, runtime_config: &RuntimeConfig) -> OcrResult {
    let dto_response = start_ipc_socketpair(req, runtime_config).unwrap();
    OcrResult::from(&dto_response)
}