import socket
import sys

def main():
    socket_path = "/tmp/my_socket_bryan"
    
    try:
        # Connect to server
        client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        client.connect(socket_path)
        print(f"✓ Connected to {socket_path}")
        
        # Send message
        message = b"hello from python"
        client.sendall(message)
        print(f"→ Sent: {message.decode()}")
        
        # Receive response
        response = client.recv(1024)
        print(f"← Response: {response.decode()}")
        
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
        try:
            client.close()
            print("✓ Connection closed")
        except:
            pass

if __name__ == "__main__":
    main()