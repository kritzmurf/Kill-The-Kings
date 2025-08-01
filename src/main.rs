#[allow(unused)]

mod ui;

use crate::ui::app::{App, CurrentScreen};
use crate::ui::ui::{render};
//use crate::ui::ui::{render, render_size_error};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
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

//is there a way to get "full screen"?
pub const MIN_WIDTH: u16 = 132;
pub const MIN_HEIGHT: u16 = 46;

fn main() ->Result<(), io::Error> {
    //boilerplate to set up the terminal
    let mut terminal = ratatui::init(); 

    let mut app = App::new(); 
    execute!(io::stdout(), EnableMouseCapture)?;
    run(&mut terminal, &mut app)?;
    execute!(io::stdout(), DisableMouseCapture)?;
    Ok(())
}

fn run(terminal: &mut Terminal, app: &mut App) -> io::Result<bool> {
    terminal.draw(|f| render(f, app));
    
}


#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, Kill The Kings!", hello_world());
    }
}
