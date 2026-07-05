use serde::Deserialize;
use std::io::Read;
use std::os::unix::net::UnixStream;
use crate::ocr::dto::error::ResponseError;

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
/// make it a trait instead of a function
pub fn receive_response(socket: &mut UnixStream) -> Result<OcrResponse, ResponseError> {
    let mut buff = [0u8; 4];
    socket.read_exact(&mut buff)?;
    let n_bytes = u32::from_be_bytes(buff) as usize;

    let mut buff_res = vec![0u8; n_bytes];
    socket.read_exact(&mut buff_res)?;

    let response: OcrResponse = serde_json::from_slice(&buff_res)?;
        //.expect("Error al deserealizar respuesta");
    Ok(response)
}