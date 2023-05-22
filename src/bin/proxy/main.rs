use clap::StructOpt;
mod cli;
use tcp_relay_rust::{RelayError, Socket, StdOrSocket};

#[tokio::main]
async fn main() -> Result<(), Box<RelayError>> {
    let args = cli::Cli::parse();
    println!("proxying {:?} to {}", args.local_host, args.remote_host);
    let remote_socket: Socket = Socket::try_from(args.remote_host)?;
    let stdorsocket = match args.local_host {
        Some(local_host) => StdOrSocket::Socket(Socket::try_from(local_host)?),
        None => StdOrSocket::Std,
    };
    #[cfg(unix)]
    if let StdOrSocket::Socket(Socket::Unix(path)) = &stdorsocket {
        use std::{fs, path::Path};
        if Path::new(&path).exists() {
            println!("path {path} exists, deleting file");
            fs::remove_file(path).expect("unable to delete file");
        }
    }
    stdorsocket.run(remote_socket).await.expect("");
    Ok(())
}
