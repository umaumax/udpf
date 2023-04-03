# udpf

udp port forwarding command

## What is this?
This command can do udp port forwarding.

``` mermaid
sequenceDiagram
    participant src-host
    participant udpf-host
    participant dst-host
    src-host->>udpf-host: [client port]send data A[listen port]
    udpf-host->>dst-host: [some port]send data A[forwarding port]
    dst-host->>udpf-host: [some port]send data B[forwarding port]
    udpf-host->>src-host: [client port]send data B[listen port]
```

## how to use
``` bash
$ cargo run
[2023-04-03T13:59:32Z INFO  udpf] Listening on: 0.0.0.0:10010
[2023-04-03T13:59:32Z INFO  udpf] Forwarding to: localhost:10020
[2023-04-03T13:59:35Z INFO  udpf] bind 127.0.0.1:58454 access to 127.0.0.1:65178
[2023-04-03T13:59:35Z INFO  udpf] 127.0.0.1:58454 <--> [0.0.0.0:10010 127.0.0.1:65178] <--> localhost:10020
```

``` bash
$ cd tests
$ ./rx.py -t localhost -p 10020
127.0.0.1:58454 wrote:
b'Hello, World!'
```

``` bash
$ cd tests
$ ./tx.py -t localhost -p 10010
UDP target IP: localhost
UDP target port: 10010
message: b'Hello, World!'
got: b'HELLO, WORLD!' from ('127.0.0.1', 10010)
```

