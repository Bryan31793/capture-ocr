use thiserror::Error;
use core::fmt;
use crate::ocr::dto::error::{RequestError, ResponseError};

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("Creation of sockets failed")]
    SocketInit(#[from] std::io::Error),

    //this error could implement source and show the socket fd
    #[error("Write to socket failed")]
    SocketWrite(#[from] RequestError),

    #[error("Socket didn't received a valid response")]
    SocketRead(#[from] ResponseError),

    #[error("IPC failed")]
    SocketIpc(#[from] ProcessError),

    #[error("Failed at reading flags")]
    SocketFcntl(),
}

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Process exit status failed: {process_data}\nexit code: {exit_code}")]
    ProcessExitFailed {
        process_data: ProcessData,
        exit_code: i32,
    },

    #[error("Spawn of child process failed")]
    SpawnProcess {
        process_data: ProcessData,

        #[source]
        source: std::io::Error,
    },

    #[error("Wait for process to exit failed")]
    WaitProcess {
        process_data: ProcessData,

        #[source]
        source: std::io::Error,
    }
}

#[derive(Debug)]
pub struct ProcessData {
    pub program: String,
    pub path: String,
}

impl ProcessData {
    pub fn new(program: String, path: String) -> Self {
        ProcessData { program, path }
    }
}

impl fmt::Display for ProcessData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program: {}\nPath: {}", self.program, self.path)
    }
}