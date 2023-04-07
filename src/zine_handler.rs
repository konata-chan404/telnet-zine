use crate::{telnet::TelnetHandler, zine::Magazine};
use crossterm::{
    cursor::MoveTo,
    style::{style, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::Result;

#[derive(Clone, Debug)]
enum ZineState {
    Front,
    Reading {
        section: usize,
        page: usize,
    },
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

    // Utility function to clear the screen
    fn clear_screen(&self) -> String {
        // Create a `Clear(ClearType::All)` command and move the cursor to the top-left position
        let output = format!("{}", Clear(ClearType::All));
        format!("{}{}", MoveTo(0, 0), output)
    }

    // Display the front page of the zine
    fn display_front_page(&self) -> String {
        let front_text = self.magazine.front_text.clone();
        let styled_output = style(front_text).on_black();
        format!("{}{}\r\n", self.clear_screen(), styled_output)
    }

    // Display a specific page in a section of the zine
    fn display_section_page(&self, section: usize, page: usize) -> String {
        match self.magazine.get_section(section) {
            Some(sec) => {
                if let Some(text) = sec.get_page(page) {
                    let styled_output = style(text).on_black();
                    format!("{}{}\r\n", self.clear_screen(), styled_output)
                } else {
                    format!("Section {} does not have a page {}\r\n", section + 1, page + 1)
                }
            }
            None => format!("Section {} does not exist\r\n", section + 1),
        }
    }
}

impl TelnetHandler for ZineHandler {
    // When a telnet client connects, display the cover page of the zine
    fn on_connect(&mut self) -> String {
        let cover_text = self.magazine.cover_text.clone();
        let styled_output = style(cover_text).on_black();
        let output = format!("{}\r\n", styled_output);
        format!("{}{}", self.clear_screen(), output)
    }

    // Handle input from the telnet client
    fn handle(&mut self, input: &str) -> String {
        if input == "x" || input == "X" || input == "exit" || input == "quit" {
            return self.quit();
        }
        match self.state {
            ZineState::Front => {
                // If the zine is on the front page and the input is a valid section index,
                // start reading that section from the first page
                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= self.magazine.sections.len() {
                        self.state = ZineState::Reading {
                            section: index - 1,
                            page: 1,
                        };
                        self.display_section_page(index - 1, 0)
                    } else {
                        "".to_string()
                    }
                } else {
                    // Otherwise, just display the front page
                    self.display_front_page()
                }
            }
            ZineState::Reading { section, page } => {
                let section_len = self.magazine.get_section(section).map(|s| s.pages.len()).unwrap_or(0);
                let next_page = (page + 1);
                
                if page == section_len || next_page > section_len {
                    self.state = ZineState::Front;
                    return format!("{}Press ENTER to go back to front page.", self.clear_screen());
                }
                else {
                    self.state = ZineState::Reading { section: section, page: next_page };
                    return self.display_section_page(section, page)
                }
            }
        }
    }
}