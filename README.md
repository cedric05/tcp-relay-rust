Simple tcp relay using rust and tokio

## Build

`cargo build --release`

## Run

To create local tcp relay to `google.com:443` from `localhost:3333`. 

`./target/release/tcp-relay-rust google.com:443 localhost:3333`

To try:

    curl https://localhost:3333/ -k

### Tcp Example

- start redis server using: `docker run --rm -p 6379:6379 redis`
- start relay service: `./target/release/tcp-relay-rust localhost:6379 localhost:3333`
- start client by connecting to `redis-cli -p 3333`

### Unix Example

- start relay service: `./target/release/tcp-relay-rust /var/run/docker.sock localhost:3333` (danger, its not safe to share docker.sock)
- invoke sample request by `curl localhost:3333`

### Std Example

- start relay service: `./target/release/tcp-relay-rust /var/run/docker.sock ` (danger, its not safe to share docker.sock)
- type below text. (docker.sock will respond with text)
"""
GET /containers/json HTTP/1.1
Host: localhost:3333
"""