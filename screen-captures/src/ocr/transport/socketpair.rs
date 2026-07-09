use crate::ocr::dto::request::OcrRequest;
use crate::ocr::dto::response::{receive_response, OcrResponse};
use crate::ocr::transport::error::{ProcessError, ProcessData, SocketError};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::thread;

/// Start the ipc using socketpair
/// TODO:
/// separate hard coded paths into a different file
pub fn start_ipc_socketpair(request: &OcrRequest) -> Result<OcrResponse, SocketError> {
    let (mut sock1, sock2) = UnixStream::pair()?;

    //fd must surivive exec()
    let fd = sock2.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        if flags == -1 {
            return Err(SocketError::SocketFcntl());
        }
        libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
    }

    let socket2_fd = sock2.as_raw_fd().to_string();
    let path = String::from("/home/bryan/capture-ocr/ocr/main.py");
    let venv_python = std::env::var("OCR_PYTHON")
        .unwrap_or_else(|_| "/home/bryan/capture-ocr/ocr/.venv_paddleocr/bin/python3".to_string());

    thread::spawn(move || -> Result<(), ProcessError> {
        spawn_process(&venv_python, &path, &socket2_fd)?;
        Ok(())
    });
    
    request.send_request(&mut sock1)?;
    let response = receive_response(&mut sock1)?;
    
    drop(sock2);
    Ok(response)
}

/// Create child process in charge of OCR
fn spawn_process(program: &String, path: &String, socket_fd: &String) -> Result<(), ProcessError>{
    let mut child = Command::new(program)
        .args([path, socket_fd])
        .spawn()
        .map_err(|err| ProcessError::SpawnProcess { 
            process_data: ProcessData::new(String::from(program), String::from(path)),
            source: err 
        })?;

    let status = child.wait()
        .map_err(|err| ProcessError::WaitProcess { 
            process_data: ProcessData::new(String::from(program), String::from(path)),
            source: err 
        })?;

    if status.success() {
        return Ok(());
    } else {
        return Err(
            ProcessError::ProcessExitFailed {
                process_data: ProcessData::new(String::from(program), String::from(path)),
                exit_code: status.code().unwrap_or(-1),
            }
        );
    }
}