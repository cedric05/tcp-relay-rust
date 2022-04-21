use clap::StructOpt;
use tcp_relay_rust::{cli, RelayError, Socket};

#[tokio::main]
async fn main() -> Result<(), Box<RelayError>> {
    let args = cli::Cli::parse();
    println!("proxying {} to {}", &args.local_host, &args.remote_host);
    let remote_socket: Socket = Socket::try_from(args.remote_host)?;
    let local_socket: Socket = Socket::try_from(args.local_host)?;
    match &local_socket {
        #[cfg(unix)]
        Socket::Unix(path) => {
            use std::{fs, path::Path};
            if Path::new(&path).exists() {
                println!("path {path} exists, deleting file");
                fs::remove_file(path).expect("unable to delete file");
            }
        }
        Socket::Tcp(_) => {}
    }
    local_socket.run(remote_socket).await.expect("");
    Ok(())
}
