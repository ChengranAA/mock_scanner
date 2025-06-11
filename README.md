# Mock Scanner 2.0
A lightweight, cross-platform tool designed to simulate scanner triggers automatically. It starts on launch and can be seamlessly embedded into any experimental framework that supports standard socket functions.

## Installation
You can download from the releases or install from my homebrew tap
```bash
brew tap ChengranAA/neuraltools
brew install mock_scanner
```

## Usage
Add the following snippet to your experiment scripts


For python:
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
For MATLAB:
```MATLAB
function start_scanner(host, port)
    if nargin < 1
        host = '127.0.0.1';
    end
    if nargin < 2
        port = 2333;
    end

    t = tcpclient(host, port);
    write(t, uint8('Start'));
    clear t;
end

start_scanner();
% rest of your code
```
