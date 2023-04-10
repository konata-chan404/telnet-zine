# Telzine

Telzine is a Telnet server that serves ezines (electronic magazines) written in text format. It parses a custom structure (documented and shown in examples) and uses [Handlebars](https://docs.rs/handlebars/latest/handlebars/) for templating. The server is written in Rust and supports asynchronous I/O.

It uses ANSI features that might not be ideal for some terminal emulators.
![telzine in action](https://cdn.discordapp.com/attachments/792357670920519692/1094844040173854811/16811015575885413.gif)

## Features

- Serves ezines over Telnet protocol for that retro underground feel
- Parses a custom ezine structure with a touch of nostalgia
- Uses Handlebars for templating to add dynamic content easily
- Written in Rust for speed and safety, as an amaeture project
- Supports asynchronous I/O for scalability, because even the cutest retro things need to scale up

## Getting started

### Installing

1. Clone the repository: `git clone https://github.com/konata-chan404/telzine.git`
2. Build the server: `cargo build --release`

### Usage

1. Start the server: `./target/release/telzine`
2. Open a Telnet client and connect to the server (default port is 8080)
3. Example zine should show up, from there on it's all yours to customize~
```
telzine
USAGE:
    telzine [OPTIONS]

FLAGS:
    -h, --help       
    -V, --version    
            Prints version information

OPTIONS:
    -i, --ip <ip>                     
             [default: 127.0.0.1]
    -m, --magazine <magazine-path>    
             [default: ./examples/example-zine]
    -p, --port <port>                 
             [default: 8080]
```

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub to suggest changes or improvements, and feel free to add your own retro touches.

## License

This project is licensed under the MIT License - see the LICENSE file for details. Use it responsibly, and remember to keep it cute.
