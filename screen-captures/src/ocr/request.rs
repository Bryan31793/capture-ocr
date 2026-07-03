use serde::Serialize;
use std::io::{Result as IoResult, Write};
use std::os::unix::net::UnixStream;

#[derive(Debug, Serialize)]
pub struct OcrRequest {
    pub action: String,
    pub payload: serde_json::Value,
}

/// Serialize the request into bytes and send them through the socket.
pub fn send_request(socket: &mut UnixStream, req: &OcrRequest) -> IoResult<()> {
    let payload = serde_json::to_vec(req)?;
    let length = (payload.len() as u32).to_be_bytes();
    socket.write_all(&length)?;
    socket.write_all(&payload)?;
    Ok(())
}