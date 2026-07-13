use crate::ocr::dto::request::OcrRequest;
use crate::ocr::dto::response::{receive_response, OcrResponse};
use crate::ocr::config::runtime_config::RuntimeConfig;
use crate::ocr::transport::error::{ProcessError, ProcessData, SocketError};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::thread;

/// Start the ipc using socketpair
pub fn start_ipc_socketpair(
    request: &OcrRequest,
    runtime_config: &RuntimeConfig,
) -> Result<OcrResponse, SocketError> {
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
    let runtime_config = runtime_config.clone();

    thread::spawn(move || -> Result<(), ProcessError> {
        spawn_process(&runtime_config, &socket2_fd)?;
        Ok(())
    });
    
    request.send_request(&mut sock1)?;
    let response = receive_response(&mut sock1)?;
    
    drop(sock2);
    Ok(response)
}

/// Create child process in charge of OCR
fn spawn_process(
    runtime_config: &RuntimeConfig,
    socket_fd: &str,
) -> Result<(), ProcessError> {
    let program = runtime_config.python_path.to_string_lossy().into_owned();
    let path = runtime_config.ocr_script_path.to_string_lossy().into_owned();

    let mut child = Command::new(&runtime_config.python_path)
        .arg(&runtime_config.ocr_script_path)
        .arg(socket_fd)
        .spawn()
        .map_err(|err| ProcessError::SpawnProcess {
            process_data: ProcessData::new(program.clone(), path.clone()),
            source: err
        })?;

    let status = child.wait()
        .map_err(|err| ProcessError::WaitProcess {
            process_data: ProcessData::new(program.clone(), path.clone()),
            source: err
        })?;

    if status.success() {
        return Ok(());
    } else {
        return Err(ProcessError::ProcessExitFailed {
            process_data: ProcessData::new(program, path),
            exit_code: status.code().unwrap_or(-1),
        });
    }
}