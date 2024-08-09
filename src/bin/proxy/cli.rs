use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, author, version)]
/// Proxy for tcp/unix/std to tcp/unix
pub struct Cli {
    /// Address of the local server in `host:port` or Unix socket path.
    /// If not set, it will read from `stdin` and write to `stdout`.
    #[arg(short, long, value_name = "LOCAL")]
    pub local_host: Option<String>,

    /// Address of the remote server in `host:port` or Unix socket path to expose local ports to.
    #[arg(short, long, value_name = "REMOTE")]
    pub remote_host: String,
}
