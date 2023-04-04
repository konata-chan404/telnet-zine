mod telnet;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let handler = telnet::EchoHandler::new();
    let server = telnet::TelnetServer::new("127.0.0.1:8080", handler).await?;
    server.run().await
}

