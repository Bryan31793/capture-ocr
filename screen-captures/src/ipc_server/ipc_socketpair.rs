use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::process::Command;

pub fn start_ipc_socketpair() {
    let (mut sock1, sock2) = UnixStream::pair().expect("Failed to create socket pair");

    // Quitar O_CLOEXEC de sock2 para que el hijo lo herede tras exec
    let fd = sock2.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
    }

    let socket2_fd = sock2.as_raw_fd();

    let mut child = Command::new("python3")
        .args(["/home/bryan/capture-ocr/ocr/main.py", &socket2_fd.to_string()])
        .spawn()
        .expect("Failed to spawn process");

    sock1.write_all(b"Hola python desde rust").unwrap();

    let mut buf = [0; 100];
    let bytes_read = sock1.read(&mut buf).unwrap();
    println!("Rust recibio: {}", String::from_utf8_lossy(&buf[..bytes_read]));

    drop(sock2); // cerrar despues de comunicarse
    child.wait().unwrap();
}