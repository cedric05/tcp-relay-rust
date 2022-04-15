use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Address of the remote server in (host:port) to expose local ports to.
    #[clap{}]
    pub remote_host: String,

    /// Address of the local server in (host:port)
    #[clap{}]
    pub local_host: String,
}
