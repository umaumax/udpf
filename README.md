# udpf

udp port forwarding command

## What is this?

## how to use
``` bash
$ cargo run
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

