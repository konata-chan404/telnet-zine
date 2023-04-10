mod telnet;
mod zine;
mod zine_handler;
mod helpers;

use std::{io::Result, path::Path};
use structopt::StructOpt;

use crate::{
    zine::Magazine,
    zine_handler::{ZineHandler},
};

#[derive(StructOpt)]
#[structopt(name = "telzine")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: String,
    
    #[structopt(short = "i", long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(short = "m", long = "magazine", default_value = "./examples/example-zine")]
    magazine_path: String,

    #[structopt(short = "h", long = "help")]
    help: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.help {
        Opt::clap().write_long_help(&mut std::io::stdout()).unwrap();
        return Ok(());
    }

    let magazine = Magazine::from_directory(Path::new(&opt.magazine_path));

    let handler = ZineHandler::new(magazine);
    let server = telnet::TelnetServer::new(format!("{}:{}", opt.ip, opt.port).as_str(), handler).await?;
    server.run().await
}