mod telnet;
mod zine;
mod zine_handler;
use std::{io::Result, path::Path};

use crate::{
    zine::Magazine,
    zine_handler::{ZineHandler},
};

#[tokio::main]
async fn main() -> Result<()> {
    let magazine = Magazine::from_directory(Path::new("./examples/example-zine"));
    println!("{:#?}", magazine);

    let handler = ZineHandler::new(magazine);
    let server = telnet::TelnetServer::new("127.0.0.1:8080", handler).await?;
    server.run().await
}