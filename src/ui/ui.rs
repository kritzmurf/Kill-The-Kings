//!:

// ratatui
use ratatui::Frame;
use image::imageops::FilterType;
use ratatui::buffer::Buffer;
use ratatui::layout::{
    Alignment, Constraint, Direction, Layout, Margin, Offset, Position, Rect, Size,
};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, Cell, Clear, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table,
    Wrap,
};

// ratatui_image
use ratatui_image::picker::Picker;
use ratatui_image::protocol::ImageSource;
use ratatui_image::{Image, Resize, StatefulImage};

// std
use std::cmp::min;
use std::ops::Add;
use std::rc::Rc;

// local
use include_dir::{include_dir, Dir};
use crate::ui::app::{App, CurrentScreen};

const SCREEN_SIZE: u16 = 15;

pub fn render(frame: &mut Frame, app: &mut App) {
    //TODO: 1)globally split the screen if needed (if we are going to have UI
    //      present for all screens, cut it up here before handing off the 
    //      split object
    //
    //TODO: 2)if any of those fields are drawn universally, draw them here 
    //      and only hand off the relevant pieces to the current screen detector
    match app.current_screen {
        CurrentScreen::Main => render_main(frame, Rect::new()), //TODO: define area
        CurrentScreen::Gameplay => render_gameplay(frame, Rect::new()),
        CurrentScreen::GameOver => render_game_over(frame, Rect::new()),
        CurrentScreen::About => render_about(frame, Rect::new()),
    }
    //TODO: Handle all input after the draw. Input should modify state, and the
    //next draw will reflect the state change
}

fn render_gameplay(frame: &mut Frame, app: &mut App, area: Rect) {
    //should I make a Result? Probably not unless I think it CAN fail...
    println!("hello render game screen");
}

fn render_main(frame: &mut Frame, app: &mut App, area: Rect) {
    println!("hello title screen");
}

fn render_game_over(frame: &mut Frame, app: &mut App, area: Rect) {
    println!("hello game over");
}

fn render_about(frame: &mut Frame, app: &mut App, area: Rect) {
    println!("hello about");
}
