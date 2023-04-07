use crate::{zine::Magazine, telnet::TelnetHandler};
use crossterm::{
    cursor::MoveTo,
    style::{style, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::Result;

#[derive(Clone, Debug)]
enum ZineState {
    Front,
    Reading,
}

#[derive(Clone, Debug)]
pub struct ZineHandler {
    magazine: Magazine,
    state: ZineState,
}

impl ZineHandler {
    pub fn new(zine: Magazine) -> Self {
        ZineHandler {
            state: ZineState::Front,
            magazine: zine,
        }
    }

    fn clear_screen(&self) -> String {
        let output = format!("{}", Clear(ClearType::All));
        format!("{}{}", MoveTo(0, 0), output)
    }
}

impl TelnetHandler for ZineHandler {
    fn on_connect(&mut self) -> String {
        let cover_text = self.magazine.cover_text.clone();
        let styled_output = style(cover_text).on_black();
        let output = format!("{}\r\n", styled_output);
        format!("{}{}", self.clear_screen(), output)
    }

    fn handle(&mut self, input: &str) -> String {
        match self.state {
            ZineState::Front => {
                self.state = ZineState::Reading;
                let front_text = self.magazine.front_text.clone();
                let styled_output = style(front_text).on_black();
                let output = format!("{}\r\n", styled_output);
                format!("{}{}", self.clear_screen(), output)
            }
            ZineState::Reading => {
                // Handle input based on current page
                // ...
                // Update current page as necessary
                // ...
                // Return response
                input.to_string() + "\r\n"
            }
        }
    }
}