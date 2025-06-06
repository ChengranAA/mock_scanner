# Mock Scanner 2.0
A lightweight, cross-platform tool designed to simulate scanner triggers automatically. It starts on launch and can be seamlessly embedded into any experimental framework that supports standard socket functions.

## Usage
Add the following snippet to your experiment scripts (in python, you can find more language in the example folder)
```python
import socket

def start_scanner(host = "127.0.0.1", port = 2333):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(b"Start")

if __name__ == "__main__":
    start_scanner()
    # rest of your code
```
