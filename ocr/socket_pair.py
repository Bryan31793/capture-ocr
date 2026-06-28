import socket
import sys
import os

def socket_pair_process():
    if len(sys.argv) < 2:
        print("Error: FD requerido", file=sys.stderr)
        sys.exit(1)

    fd = int(sys.argv[1])

    # fromfd duplica el FD internamente
    sock = socket.fromfd(fd, socket.AF_UNIX, socket.SOCK_STREAM)
    os.close(fd)  # cerrar el FD original ya que fromfd lo duplica

    try:
        mensaje_rust = sock.recv(1024).decode('utf-8')
        print(f"Python recibio: {mensaje_rust}")
        sock.sendall(b"Saludos desde Python")
    finally:
        sock.close()