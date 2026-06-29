import socket
import sys
import json
import struct
import os

def socket_process():
    #To DO clean socket
    socket_path = "/tmp/my_socket_bryan"

    # Limpiar socket anterior si existe
    if os.path.exists(socket_path):
        os.unlink(socket_path)
    
    client = None
    try:
        # Connect to server
        client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        client.connect(socket_path)
        print(f"✓ Connected to {socket_path}")
        
        # Keep listening for requests from the Rust server
        # The server will send requests, and we respond
        while True:
            try:
                handle_client(client)
            except EOFError:
                # Connection closed by server
                print("✓ Server closed connection")
                break
            except Exception as e:
                print(f"✗ Error handling request: {e}")
                break
        
    except FileNotFoundError:
        print(f"✗ Error: Socket not found at {socket_path}")
        print("  Make sure the Rust server is running")
        sys.exit(1)
    except ConnectionRefusedError:
        print(f"✗ Error: Connection refused by {socket_path}")
        sys.exit(1)
    except Exception as e:
        print(f"✗ Error: {e}")
        sys.exit(1)
    finally:
        if client:
            try:
                client.close()
                print("✓ Connection closed")
            except:
                pass

def send_response(conn: socket, response: dict):
    payload = json.dumps(response).encode("utf-8")
    length = struct.pack(">I", len(payload))  # 4 bytes big-endian
    conn.sendall(length + payload)


def recv_request(conn: socket) -> dict:
    raw_len = conn.recv(4)
    if not raw_len:  # Connection closed
        raise EOFError("Connection closed by server")
    length = struct.unpack(">I", raw_len)[0]
    data = conn.recv(length)
    if not data:  # Connection closed
        raise EOFError("Connection closed by server")
    return json.loads(data.decode("utf-8"))


def handle_client(conn: socket):
    try:
        request = recv_request(conn)
        #result = do_work(request)  # tu lógica OCR, etc.
        if request:
            result = "bien hecho compa"
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