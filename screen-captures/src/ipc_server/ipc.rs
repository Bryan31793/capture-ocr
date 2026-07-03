/// IPC Server - Inter-process communication server using Unix Sockets
/// 
/// This file creates a server that listens on a Unix socket and receives messages
/// from clients (like Python scripts), responding to each one

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write, Result as IoResult};
use std::process::Command;  
use std::thread;
use std::time::Duration;
use crate::ipc_server::socket_config::SocketConfig;
use crate::ocr::{OcrRequest, OcrResponse};

// Starts the Inter-process Communication between rust and python
/// TODO:
/// message will be a OcrRequest
/// refactorize listener into a create_socket_server fn
/// refactorize stream into a create_stream_socket
pub fn start_ipc(socket_config: &SocketConfig, req: &OcrRequest) {
    // que pasa si tengo dos procesoss con el mismo socket_path??
    let _ = std::fs::remove_file(socket_config.path());

    
    let listener = match UnixListener::bind(socket_config.path()) {
        Ok(l) => {
            println!("Conexion exitosa desde rust");
            l
        }
        Err(e) => {
            eprintln!("Error al crear socket: {}", e);
            return;
            //podria regresar un bool
        }
    }; 

    //let listener = UnixListener::bind(socket_config.path())?;

    let program = socket_config.program().to_string();
    let process_path = socket_config.process_path().to_string();
    thread::spawn(move || {
        spawn_python_client(&program, &process_path);
    });
 
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                match handle_client(&mut stream, req) {
                    Ok(response) => {
                        match response {
                            OcrResponse::Ok {data} => {
                                println!("Bien hecho amigo Bryan");
                            }
                            OcrResponse::Error { code, message, retryable } => {
                                eprintln!("Error [{code}]: {message}");
                                if retryable.unwrap_or(false) {
                                    println!("No se puede recuperar del error");
                                }
                            }
                        }

                    }
                    Err(e) => {
                        println!("Error: {}", e)
                    }
                }
            }

            Err(e) => {
                println!("Error en la comunicacion: {}", e);
            }
        }
    } 
}


/// Launches the Python client as a child process
/// 
/// This process runs independently
/// If it fails, it prints the error but doesn't affect the server
/// TODO:
/// a better aproach for the nested match
fn spawn_python_client(program: &str, process_path: &str) {
    thread::sleep(Duration::from_secs(1));

    println!("Starting Python client...\n");

    match Command::new(program)
        .arg(process_path)
        .spawn() 
    {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!("Proceso de python finalizo exitosamente");
                    } else {
                        //cual podria ser el caso de este error?
                        println!("Proceso de python finalizo con error");
                    }
                }
                Err(e) => {
                    println!("Error en proceso de python: {}", e);
                }

            }
        }
        Err(e) => {
            println!("Error esperando proceso de python: {}", e);
        }
    }
}

/// Function to handle communication with ONE client
/// 
/// # Parameters
/// - stream: The bidirectional connection with the client
/// - message_screenshot: screenshots path
/// TODO: change message_screenshot from str to OcrRequest
fn handle_client(stream: &mut UnixStream, req: &OcrRequest) -> IoResult<OcrResponse> {
    // Enviar: length prefix + payload
    let payload = serde_json::to_vec(req)?;
    let length = (payload.len() as u32).to_be_bytes();
    stream.write_all(&length)?;
    stream.write_all(&payload)?;

    // Recibir: leer length prefix primero
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let resp_len = u32::from_be_bytes(len_bytes) as usize;
    
    // Leer exactamente esa cantidad de bytes
    let mut resp_buf = vec![0u8; resp_len];
    stream.read_exact(&mut resp_buf)?;

    let response: OcrResponse = serde_json::from_slice(&resp_buf)?;
    Ok(response)
}

/* 
fn send_request(stream: &mut UnixStream, req: &OcrRequest) -> IoResult<()> {
    let payload = serde_json::to_vec(req)?;
    let length = (payload.len() as u32).to_be_bytes();
    stream.write_all(&length)?;
    stream.write_all(&payload)?;
    Ok(())
} */