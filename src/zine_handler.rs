use crate::{zine::Magazine, telnet::TelnetHandler};
use crossterm::{
    cursor::MoveTo,
    style::{style, Stylize},
    terminal::{Clear, ClearType},
};

#[derive(Clone, Debug)]
enum ZineState {
    Cover,
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
            state: ZineState::Cover,
            magazine: zine,
        }
    }
}

impl TelnetHandler for ZineHandler {
    fn handle(&mut self, input: &str) -> String {
        match self.state {
            ZineState::Cover => {
                self.state = ZineState::Front;
                let cover_text = self.magazine.cover_text.clone();
                let styled_output = style(cover_text).on_black();
                let output = format!("{}{}\r\n", Clear(ClearType::All), styled_output);
                format!("{}{}", MoveTo(0, 0), output)
                //output
            }
            ZineState::Front => {
                self.state = ZineState::Reading;
                let front_text = self.magazine.front_text.clone();
                let styled_output = style(front_text).on_black();
                let output = format!("{}{}\r\n", Clear(ClearType::All), styled_output);
                format!("{}{}", MoveTo(0, 0), output)
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