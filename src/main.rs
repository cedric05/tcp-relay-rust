use core::panic;

use std::net::ToSocketAddrs;
// use std::time::Duration;

use clap::StructOpt;
use tokio::net::TcpListener;
// use tokio::time::timeout;
use tokio::{io, net::TcpSocket};
mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    let listener = TcpListener::bind((args.local_host, args.local_port)).await?;
    let mut addrs = ((args.remote_host, args.remote_port)).to_socket_addrs()?;
    let remote_addr = match addrs.next() {
        Some(remote_addr) => remote_addr,
        None => panic!("Not able to resolve remove remote addr"),
    };
    let socket_type = if remote_addr.is_ipv4() {
        TcpSocket::new_v4
    } else {
        TcpSocket::new_v6
    };

    loop {
        let socket = listener.accept().await;
        match socket {
            Ok((local_stream, client_addr)) => {
                println!("Recieved new connection: {}", client_addr);
                tokio::spawn(async move {
                    let socket = socket_type().unwrap();
                    let remote_stream = socket.connect(remote_addr).await.unwrap();
                    let (mut local_read, mut local_write) = io::split(local_stream);
                    let (mut remote_read, mut remote_write) = io::split(remote_stream);
                    tokio::select! {
                        _=io::copy(&mut local_read, &mut remote_write)=>{}
                        _=io::copy(&mut remote_read,&mut local_write)=>{}
                    }
                    println!("connection: {:?} is closed", client_addr);
                });
            }
            Err(err) => {
                println!("error is {err}")
            }
        }
    }
}
