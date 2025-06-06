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
