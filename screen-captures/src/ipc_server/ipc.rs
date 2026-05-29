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

// Starts the Inter-process Communication between rust and python
pub fn start_ipc(socket_config: &SocketConfig) {
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

    let program = socket_config.program().to_string();
    let process_path = socket_config.process_path().to_string();
    thread::spawn(move || {
        spawn_python_client(&program, &process_path);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                match handle_client(&mut stream) {
                    Ok(_) => {
                        println!("comunicacion exitosa");
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
fn handle_client(stream: &mut UnixStream) -> IoResult<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    let n_bytes = stream.read(&mut buffer)?;

    let message = String::from_utf8_lossy(&buffer[..n_bytes]);
    println!("Recibido desde rust: {}", message);

    stream.write_all(b"Ok")?;

    Ok(())
}