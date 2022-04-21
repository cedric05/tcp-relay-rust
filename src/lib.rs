use std::fmt::Debug;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
#[cfg(unix)]
use std::path::Path;

use tokio::io;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::{TcpListener, TcpStream};

pub mod cli;

#[derive(Clone)]
pub enum Socket {
    #[cfg(unix)]
    Unix(String),
    Tcp(SocketAddr),
}

enum SocketStream {
    #[cfg(unix)]
    Unix(tokio::net::UnixStream),
    Tcp(TcpStream),
}

enum SocketListener {
    #[cfg(unix)]
    Unix(tokio::net::UnixListener),
    Tcp(TcpListener),
}

pub struct RelayError(String);

impl Debug for RelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelayError")
            .field("error", &self.0)
            .finish()
    }
}

impl TryFrom<String> for Socket {
    type Error = RelayError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_socket_addrs() {
            Ok(mut socket_addr) => Ok(Socket::Tcp(socket_addr.next().unwrap())),
            Err(err) => {
                #[cfg(unix)]
                if cfg!(unix) {
                    let path = &Path::new(&value);
                    return match path.exists() || path.parent().unwrap().exists() {
                        true => Ok(Socket::Unix(value)),
                        false => Err(RelayError(format!(
                            "tcp failed with {}, unix socket will as both parent dir and current file does't exist",
                            err
                        ))),
                    };
                }
                Err(RelayError(format!("parsing failed with error {}", err)))
            }
        }
    }
}

impl Socket {
    async fn connect(&self) -> anyhow::Result<SocketStream> {
        match self {
            #[cfg(unix)]
            Socket::Unix(path) => Ok(SocketStream::Unix(
                tokio::net::UnixStream::connect(path).await?,
            )),
            Socket::Tcp(addr) => Ok(SocketStream::Tcp(TcpStream::connect(addr).await?)),
        }
    }

    async fn accept(&self, listener: &SocketListener) -> anyhow::Result<SocketStream> {
        match listener {
            #[cfg(unix)]
            SocketListener::Unix(unixlistener) => {
                let (listener, addr) = unixlistener.accept().await?;
                println!("recieved connection from {:?}", &addr);
                Ok(SocketStream::Unix(listener))
            }
            SocketListener::Tcp(tcplistener) => {
                let (listener, addr) = tcplistener.accept().await?;
                println!("recieved connection from {:?}", &addr);
                Ok(SocketStream::Tcp(listener))
            }
        }
    }

    async fn listen(&self) -> anyhow::Result<SocketListener> {
        match self {
            #[cfg(unix)]
            Socket::Unix(path) => Ok(SocketListener::Unix(tokio::net::UnixListener::bind(path)?)),
            Socket::Tcp(addr) => Ok(SocketListener::Tcp(TcpListener::bind(addr).await?)),
        }
    }

    pub async fn run(self, remote: Self) -> anyhow::Result<()> {
        let socket_listener = self.listen().await?;
        loop {
            let socket_stream = match self.accept(&socket_listener).await {
                Ok(socket_stream) => socket_stream,
                Err(accept_error) => {
                    println!("accpeting socket failed with error {}", accept_error);
                    continue;
                }
            };
            let remote_stream = match remote.clone().connect().await {
                Ok(remote_stream) => remote_stream,
                Err(accept_error) => {
                    println!(
                        "connecting to remote socket failed with error {}",
                        accept_error
                    );
                    continue;
                }
            };
            tokio::spawn(socket_stream.proxy(remote_stream));
        }
    }
}

pub async fn proxy<T1, T2>(s1: T1, s2: T2)
where
    T1: AsyncRead + AsyncWrite + Unpin,
    T2: AsyncRead + AsyncWrite + Unpin,
{
    let (mut read_1, mut write_1) = io::split(s1);
    let (mut read_2, mut write_2) = io::split(s2);
    tokio::select! {
        _=io::copy(&mut read_1, &mut write_2)=>{},
        _=io::copy(&mut read_2, &mut write_1)=>{}
    }
    println!("closing connection");
}

impl SocketStream {
    async fn proxy(self, socket2: SocketStream) {
        match (self, socket2) {
            #[cfg(unix)]
            (SocketStream::Unix(unixstream), SocketStream::Tcp(tcpstream)) => {
                proxy(unixstream, tcpstream).await;
            }
            #[cfg(unix)]
            (SocketStream::Tcp(tcpstream), SocketStream::Unix(unixstream)) => {
                proxy(tcpstream, unixstream).await;
            }
            #[cfg(unix)]
            (SocketStream::Unix(unixstream), SocketStream::Unix(unixstream2)) => {
                proxy(unixstream, unixstream2).await;
            }
            (SocketStream::Tcp(tcpstream), SocketStream::Tcp(tcpstream2)) => {
                proxy(tcpstream, tcpstream2).await;
            }
        }
    }
}
