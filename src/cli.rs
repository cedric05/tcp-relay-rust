use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Proxy for tcp/unix/std to tcp/unix
pub struct Cli {
    /// Address of the remote server in (host:port)/Unix socket path to expose local ports to.
    #[clap{}]
    pub remote_host: String,

    /// Address of the local server in (host:port)/Unix socket path.
    ///  if not set, it will read from stdin and write to stdout
    #[clap{}]
    pub local_host: Option<String>,
}
