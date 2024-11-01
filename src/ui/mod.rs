pub mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use widgets::TextField;

pub fn ui(frame: &mut Frame) {
    let [header, main, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(frame.area());
}
