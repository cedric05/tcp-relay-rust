use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Optional port on the local server to select.
    #[clap(long, default_value_t = 3333)]
    pub local_port: u16,

    /// The local host to expose.
    #[clap(short, long, value_name = "HOST", default_value = "localhost")]
    pub local_host: String,

    /// Address of the remote server to expose local ports to.
    #[clap{}]
    pub remote_host: String,

    /// Optional port on the remote server to select.
    #[clap(long, default_value_t = 80)]
    pub remote_port: u16,

    // Local connection timeout in millis.
    // #[clap(long, default_value_t = 15000)]
    // pub timeout: u64,
}
