use crate::ui;
use include_dir::{include_dir, Dir};

use ratatui::{
    Frame,
    DefaultTerminal,
};
use std::{io};

//adding art assets to binary
static RESOURCES: Dir<'_> = include_dir!("resources");

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Default)]
pub struct App {
    //screen we are displaying
    current_screen: CurrentScreen,
    should_quit: bool,

    //mouse
    mouse_position: Point,
}

impl App {
    //set up the game
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events();
        }
        Ok(())
    }
    fn draw(&self, frame &mut Frame) {}
    fn handle_events() -> io::Result<()>{
        Ok(())
    }
}

fn get_image(path: &str) ->Vec<u8> {
    if let Some(img) == RESOURCES.get_file(path).map(|file| file.contents()) {
        img.to_vec()
    } else {
        vec[] //should we make this a result err state?
    }
}

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main, //Main splash Screen
    Gameplay, //Main Gameplay Screen
    GameOver, //Screen When lost. NOTE: Maybe just a popup?
    About, //Screen about the product
}
