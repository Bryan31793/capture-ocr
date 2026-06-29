use serde::{Deserialize, Serialize};
use std::io::{Read, Result as IoResult, Write};
use std::os::unix::net::UnixStream;

#[derive(Serialize)]
pub struct OcrRequest {
    pub action: String,
    pub payload: serde_json::Value,
}
    
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum OcrResponse {
    Ok {
        data: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
        retryable: Option<bool>,
    }
}

/// Serialize the request into bytes and send them through the socket
/// TODO:
/// change the Err
pub fn send_request(socket: &mut UnixStream, req: &OcrRequest) -> IoResult<()> {
    let payload = serde_json::to_vec(req)?;
    let lenght = (payload.len() as u32).to_be_bytes();
    socket.write_all(&lenght)?;
    socket.write_all(&payload)?;
    Ok(())
}

/// Receive bytes from the socket and deserailize them into the OcrResponse struct
/// TODO:
/// change the Err
pub fn receive_response(socket: &mut UnixStream) -> IoResult<OcrResponse> {
    let mut buff = [0u8; 4];
    socket.read_exact(&mut buff)?;
    let n_bytes = u32::from_be_bytes(buff) as usize;

    let mut buff_res = vec![0u8; n_bytes];
    socket.read_exact(&mut buff_res)?;

    let response: OcrResponse = serde_json::from_slice(&buff_res).expect("Error al deserealizar respuesta");
    Ok(response)
}
