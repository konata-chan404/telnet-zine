use std::io::Result;
use std::io::ErrorKind::WouldBlock;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Define the TelnetServer struct that wraps a TcpListener and a TelnetHandler
pub(crate) struct TelnetServer<H: TelnetHandler> {
    listener: TcpListener,
    handler: H,
}

// Implement the TelnetServer struct with generic parameters and methods
impl<H: TelnetHandler + Send + Sync + 'static + Clone> TelnetServer<H> {
    // Define the constructor that binds the address to the listener and returns a new TelnetServer instance
    pub async fn new(addr: &str, handler: H) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(TelnetServer { listener, handler })
    }

    // Define the async method that runs the server and spawns a new task for each incoming connection
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

// Define the TelnetSession struct that wraps a TcpStream and a TelnetHandler
struct TelnetSession<H: TelnetHandler> {
    stream: TcpStream,
    handler: H,
}

// Implement the TelnetSession struct with generic parameters and methods
impl<H: TelnetHandler + Send + Sync> TelnetSession<H> {
    // Define the constructor that creates a new TelnetSession instance from a TcpStream and a TelnetHandler
    async fn new(stream: TcpStream, handler: H) -> Self {
        TelnetSession { stream, handler }
    }

    // Define the async method that runs the TelnetSession and handles incoming messages
    async fn run(mut self) -> Result<()> {
        self.stream.write_all(self.handler.on_connect().as_bytes()).await?;
        loop {
            let mut buffer = [0u8; 1024];
            match self.stream.read(&mut buffer).await {
                Ok(0) => return Ok(()), // Connection closed by client
                Ok(n) => {
                    let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    let output = self.handler.handle(&input);
                    self.stream.write_all(output.as_bytes()).await?;
                }
                Err(ref e) if e.kind() == WouldBlock => continue, // Non-blocking error, continue looping
                Err(e) => return Err(e.into()), // Propagate any other errors
            }
        }
    }
}

// Define the TelnetHandler trait that represents the Telnet protocol message handler
pub trait TelnetHandler: Send + Sync + 'static {
    fn handle(&mut self, input: &str) -> String;

    fn on_connect(&mut self) -> String {
        "".to_string()
    }
}

// Define a simple EchoHandler struct that implements the TelnetHandler trait
#[derive(Copy, Clone, Debug)]
pub(crate) struct EchoHandler;

impl EchoHandler {
    pub fn new() -> Self {
        EchoHandler {}
    }
}

impl TelnetHandler for EchoHandler {
    // Implement the TelnetHandler trait's handle method for the EchoHandler
    fn handle(&mut self, input: &str) -> String {
        input.to_string() + "\r\n"
    }
}