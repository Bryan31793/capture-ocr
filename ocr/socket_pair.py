import socket
import sys
import os
import struct
import json
from extract_text_main import start_extract_text

def socket_pair_process():
    if len(sys.argv) < 2:
        print("Error: FD requerido", file=sys.stderr)
        sys.exit(1)

    fd = int(sys.argv[1])

    # fromfd duplica el FD internamente
    sock = socket.fromfd(fd, socket.AF_UNIX, socket.SOCK_STREAM)
    os.close(fd)  # cerrar el FD original ya que fromfd lo duplica

    try:
        handle_client(sock)
        """mensaje_rust = sock.recv(1024).decode('utf-8')
        print(f"Python recibio: {mensaje_rust}")
        sock.sendall(b"Saludos desde Python")"""
    finally:
        sock.close()


def recv_request(sock: socket) -> dict:
    """
    Receive a JSON represented in bytes and serialize them into a dict

    Args: 
        sock (socket): socket from which bytes will be read 

    Returns:
        dict: JSON represented in a python dict

    Raises:
        EOFError: if one socket is shutdown
    """
    raw_len = sock.recv(4)
    if not raw_len:  # Connection closed
        raise EOFError("Connection closed")
    length = struct.unpack(">I", raw_len)[0]
    data = sock.recv(length)
    if not data:  # Connection closed
        raise EOFError("Connection closed")
    return json.loads(data.decode("utf-8"))

def send_response(sock: socket, response: dict):
    """
    Serialize dict into a JSON str then encodes it into bytes 
    and send them through the socket

    Args: 
        sock (socket): socket from which bytes will be sended
        response (dict): response representing the JSON 

    Returns:
        None
    """
    payload = json.dumps(response).encode("utf-8")
    length = struct.pack(">I", len(payload))  # 4 bytes big-endian
    sock.sendall(length + payload)

def handle_client(conn: socket):
    try:
        request = recv_request(conn)
        #result = inicio_ocr()  #logica OCR, etc.
        if request:
            result = start_extract_text()
            #result = request["action"]
        send_response(conn, {
            "status": "ok",
            "data": result
        })
    except ValueError as e:
        send_response(conn, {
            "status": "error",
            "code": "INVALID_INPUT",
            "message": str(e),
            "retryable": False
        })
    except Exception as e:
        send_response(conn, {
            "status": "error",
            "code": "INTERNAL_ERROR",
            "message": str(e),
            "retryable": True
        })