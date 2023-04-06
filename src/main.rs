mod telnet;
mod zine;
use std::{io::Result, path::Path};

use telnet::TelnetHandler;

use crate::zine::Magazine;

#[derive(Clone, Debug)]

enum ZineState {
    Hello,
    Reading,
}

#[derive(Clone, Debug)]
struct ZineHandler {
    magazine: Magazine,
    state: ZineState
}

impl ZineHandler {
    pub fn new(zine: Magazine) -> Self {
        ZineHandler {state: ZineState::Hello, magazine: zine}
    }
}

impl TelnetHandler for ZineHandler {
    fn handle(&mut self, input: &str) -> String {
        match self.state {
            ZineState::Hello => {
                self.state = ZineState::Reading;
                "\x1B[2J\x1B[1;1H".to_owned() + &self.magazine.cover_text.clone() + "\r\n" // Clear screen + Hello Message
            },
            ZineState::Reading => {
                // Handle input based on current page
                // ...
                // Update current page as necessary
                // ...
                // Return response
                input.to_string() + "\r\n"
            },
        }
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let magazine = Magazine::from_directory(Path::new("./examples/example-zine"));
    println!("{:#?}", magazine);

    let handler = ZineHandler::new(magazine);
    let server = telnet::TelnetServer::new("127.0.0.1:8080", handler).await?;
    server.run().await
}

