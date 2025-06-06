import socket

def start_scanner(host = "127.0.0.1", port = 2333):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(b"Start")

if __name__ == "__main__":
    start_scanner()
    # rest of your code
