# udp-blast

```
udp-blast 0.1.0
Clone and send UDP packets to n destinations.

USAGE:
    udp-blast [FLAGS] [OPTIONS] --receivers <SOCKETADDR<[,]SOCKETADDR...>>

FLAGS:
    -d, --debug         Print debug satements.
    -h, --help          Prints help information
    -p, --port-reuse    Allow multiple processes to listen and balance packets on the same port. LINUX ONLY.
    -V, --version       Prints version information

OPTIONS:
    -b, --buffer <INT>                                Sets the buffer size. [default: 1500]
    -l, --listen <SOCKETADDR>                         Sets the address and port to listen on. [default: 0.0.0.0:8125]
    -r, --receivers <SOCKETADDR<[,]SOCKETADDR...>>    Sets the destinations receiving cloned UDP packets.

```

# Build
```
cargo build --release
```

# Example Usage
```
udp-blast --port-reuse --listen=0.0.0.0:8125 --receivers=127.0.0.1:8127,127.0.0.1:8128,127.0.0.1:8129,127.0.0.1:8130 # Send statsd packets to multiple destinations.
```
