use serde::Serialize;
use std::io::Write;
use std::os::unix::net::UnixStream;
use crate::ocr::dto::error::RequestError;

#[derive(Debug, Serialize)]
pub struct OcrRequest {
    pub action: String,
    pub payload: serde_json::Value,
}

impl OcrRequest {
    /// Serialize the request into bytes and send them through the socket.
    /// TODO:
    /// make it a trait instead of a method
    pub fn send_request(&self, socket: &mut UnixStream) -> Result<(), RequestError> {
        let payload = serde_json::to_vec(self)?;
        let length = (payload.len() as u32).to_be_bytes();
        socket.write_all(&length)?;
        socket.write_all(&payload)?;
        Ok(())
    }
}
