use clap::StructOpt;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    let local_addr: SocketAddr = args
        .local_host
        .to_socket_addrs()?
        .next()
        .expect("local_addr is not in `host:port`");
    let listener: TcpListener = TcpListener::bind(local_addr).await?;
    println!("proxying {} to {}", args.local_host, args.remote_host);

    loop {
        let socket = listener.accept().await;
        let remote_addr = args.remote_host.to_string();
        match socket {
            Ok((local_stream, client_addr)) => {
                println!("Recieved new connection: {}", client_addr);
                tokio::spawn(proxy(remote_addr, local_stream, client_addr));
            }
            Err(err) => {
                eprintln!("error is {err}")
            }
        }
    }
}

async fn proxy(
    remote_addr: String,
    local_stream: TcpStream,
    client_addr: SocketAddr,
) -> anyhow::Result<()> {
    let remote_stream = TcpStream::connect(remote_addr).await?;
    let (mut local_read, mut local_write) = io::split(local_stream);
    let (mut remote_read, mut remote_write) = io::split(remote_stream);
    tokio::select! {
        _ = io::copy(&mut local_read, &mut remote_write) => {}
        _ = io::copy(&mut remote_read,&mut local_write) => {}
    }
    println!("connection: {:?} is closed", client_addr);
    Ok(())
}
