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
use ratatui::Frame;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::ImageSource;
use ratatui_image::{Image, Resize, StatefulImage};
use std::cmp::min;
use std::ops::Add;
use std::rc::Rc;

use crate::ui::app::{App, CurrentScreen};
const SCREEN_SIZE: u16 = 15;

pub fn render(frame: &mut Frame, app: &mut App) {}
