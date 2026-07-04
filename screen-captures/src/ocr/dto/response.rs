use serde::Deserialize;
use std::io::{Read, Result as IoResult};
use std::os::unix::net::UnixStream;

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum OcrResponse {
    Ok {
        data: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
        retryable: Option<bool>,
    },
}

/// Receive bytes from the socket and deserialize them into an OcrResponse.
/// TODO:
/// make it a method instead of a function
pub fn receive_response(socket: &mut UnixStream) -> IoResult<OcrResponse> {
    let mut buff = [0u8; 4];
    socket.read_exact(&mut buff)?;
    let n_bytes = u32::from_be_bytes(buff) as usize;

    let mut buff_res = vec![0u8; n_bytes];
    socket.read_exact(&mut buff_res)?;

    let response: OcrResponse = serde_json::from_slice(&buff_res)
        .expect("Error al deserealizar respuesta");
    Ok(response)
}