use std::io::Result;
use std::io::ErrorKind::WouldBlock;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub(crate) struct TelnetServer<H: TelnetHandler> {
    listener: TcpListener,
    handler: H,
}

impl<H: TelnetHandler + Send + Sync + 'static + Clone> TelnetServer<H> {
    pub async fn new(addr: &str, handler: H) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(TelnetServer { listener, handler })
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening on: {}", self.listener.local_addr()?);

        loop {
            let (stream, _) = self.listener.accept().await?;
            let handler = self.handler.clone();
            tokio::spawn(async move {
                if let Err(e) = TelnetSession::new(stream, handler).await.run().await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

struct TelnetSession<H: TelnetHandler> {
    stream: TcpStream,
    handler: H,
}

impl<H: TelnetHandler + Send + Sync> TelnetSession<H> {
    async fn new(stream: TcpStream, handler: H) -> Self {
        TelnetSession { stream, handler }
    }

    async fn run(mut self) -> Result<()> {
        loop {
            let mut buffer = [0u8; 1024];
            match self.stream.read(&mut buffer).await {
                Ok(0) => return Ok(()), // Connection closed by client
                Ok(n) => {
                    let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    let output = self.handler.handle(&input);
                    self.stream.write_all(output.as_bytes()).await?;
                }
                Err(ref e) if e.kind() == WouldBlock => continue,
                Err(e) => return Err(e.into()),
            }
        }
    }
}

pub trait TelnetHandler: Send + Sync + 'static {
    fn handle(&mut self, input: &str) -> String;
}


#[derive(Copy, Clone, Debug)]
pub(crate) struct EchoHandler;

impl EchoHandler {
    pub fn new() -> Self {
        EchoHandler {}
    }
}

impl TelnetHandler for EchoHandler {
    fn handle(&mut self, input: &str) -> String {
        input.to_string() + "\r\n"
    }
}

#[derive(Copy, Clone)]
struct ReverseHandler;

impl ReverseHandler {
    fn new() -> Self {
        ReverseHandler {}
    }
}

impl TelnetHandler for ReverseHandler {
    fn handle(&mut self, input: &str) -> String {
        input.chars().rev().collect::<String>() + "\r\n"
    }
}