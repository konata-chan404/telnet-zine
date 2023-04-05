mod telnet;
mod zine;
use std::{io::Result, path::Path};

use crate::zine::Magazine;

#[tokio::main]
async fn main() -> Result<()> {
    let magazine = Magazine::from_directory(Path::new("./examples/example-zine"));
    println!("{:#?}", magazine);

    let handler = telnet::EchoHandler::new();
    let server = telnet::TelnetServer::new("127.0.0.1:8080", handler).await?;
    server.run().await
}

