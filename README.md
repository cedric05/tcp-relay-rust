Simple tcp relay using rust and tokio

## Build

`cargo build --release`

## Run

To create local tcp relay to `google.com:443` from `localhost:3333`. 

`./target/release/tcp-relay-rust google.com --remote-port 443 --local-port 3333`

To try:

    curl https://localhost:3333/ -k

### Tcp Example

- start redis server using: `docker run --rm -p 6379:6379 redis`
- start relay service: `./target/release/tcp-relay-rust localhost --remote-port 6379`
- start client by connecting to `redis-cli -p 3333`


## TODO
- support unix sockets