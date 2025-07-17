#[allow(unused)]
#![allow(unused)]

mod ui;

//use crate::ui::app::{App, CurrentScreen};
//use crate::ui::ui::{render, render_size_error};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, terminal, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal};
use std::io::{stdout, Error, ErrorKind, Stdout};
use std::{env, io, process};


pub fn hello_world() -> &'static str {
    "Hello, Kill The Kings!"
}

pub const MIN_WIDTH: u16 = 132;
pub const MIN_HEIGHT: u16 = 46;

fn require_size(terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
    let size = terminal.size()?;
    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
        terminal.clear();
        //terminal.draw(|frame| render_size_error(frame, MIN_WIDTH, MIN_HEIGHT, size))?;

        loop {
            match event::read()? {
                Event::Resize(new_width, new_height) => {
                    if new_width >= MIN_WIDTH && new_height >= MIN_HEIGHT {
                        return Ok(());
                    }
                }
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('c')
                        && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        || key.code == KeyCode::Esc
                    {
                        ratatui::restore();
                        process::exit(0);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}


fn main() ->Result<(), io::Error> {
    //boilerplate to set up the terminal
    let mut terminal = ratatui::init(); 


    let _app = App::default(); 
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, Kill The Kings!", hello_world());
    }
}
