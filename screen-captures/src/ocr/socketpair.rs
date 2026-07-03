use crate::ocr::request::{send_request, OcrRequest};
use crate::ocr::response::{receive_response, OcrResponse};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::process::Command;

pub fn start_ipc_socketpair(request: &OcrRequest) {
    let (mut sock1, sock2) = UnixStream::pair().expect("Failed to create socket pair");

    let fd = sock2.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
    }

    let socket2_fd = sock2.as_raw_fd();

    let venv_python = std::env::var("OCR_PYTHON")
        .unwrap_or_else(|_| "/home/bryan/capture-ocr/ocr/.venv_paddleocr/bin/python3".to_string());

    let mut child = Command::new(&venv_python)
        .args(["/home/bryan/capture-ocr/ocr/main.py", &socket2_fd.to_string()])
        .spawn()
        .expect("Failed to spawn process");

    send_request(&mut sock1, request).expect("Error al enviar request");

    let response = receive_response(&mut sock1).expect("Error al recibir respuesta");
    match response {
        OcrResponse::Ok { data } => {
            println!("Rust recibio: {}", data);
        }
        OcrResponse::Error { code, message, .. } => {
            println!("Error {} {}", code, message);
        }
    }

    drop(sock2);
    child.wait().unwrap();
}