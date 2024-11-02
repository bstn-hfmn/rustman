use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Styled},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};
use widgets::{CursorDirection, TextInput};

pub mod widgets;

struct CursorCoord {
    x: u16,
    y: u16,
}

pub enum OverviewArea {
    URL,
    History,
    Request,
    Response,
}

pub struct Overview {
    selected: OverviewArea,
    selection: CursorCoord,

    pub url: TextInput,
}

impl Overview {
    pub fn new() -> Self {
        Overview {
            url: TextInput::new("URL"),
            selection: CursorCoord { x: 0, y: 0 },
            selected: OverviewArea::History,
        }
    }

    fn move_selection(
        &mut self,
        dir: CursorDirection,
    ) {
        match dir {
            CursorDirection::Right => {
                self.selection.x = self.selection.x.saturating_add(1)
            }
            CursorDirection::Left => {
                self.selection.x = self.selection.x.saturating_sub(1)
            }
            CursorDirection::Down => {
                self.selection.y = self.selection.y.saturating_sub(1)
            }
            CursorDirection::Up => self.selection.y = self.selection.y.saturating_add(1),
        };

        self.selection.x = self.selection.x.clamp(0, 2);
        self.selection.y = self.selection.y.clamp(0, 1);

        self.selected = match (self.selection.x, self.selection.y) {
            (0, 0) | (0, 1) | (0, 2) => OverviewArea::History,
            (1, 0) => OverviewArea::Request,
            (2, 0) => OverviewArea::Response,
            (1, 1) | (2, 1) => OverviewArea::URL,
            _ => OverviewArea::History,
        }
    }

    pub fn handle_key_event(
        &mut self,
        ev: KeyEvent,
    ) {
        match ev.code {
            KeyCode::Right => self.move_selection(CursorDirection::Right),
            KeyCode::Left => self.move_selection(CursorDirection::Left),
            KeyCode::Down => self.move_selection(CursorDirection::Down),
            KeyCode::Up => self.move_selection(CursorDirection::Up),
            _ => {}
        }
    }
}

impl Widget for Overview {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Overview {
    fn render_ref(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let [main, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let [left, right] =
            Layout::horizontal([Constraint::Ratio(1, 4), Constraint::Fill(1)])
                .areas(main);

        let [input, content] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(right);

        let [request, response] =
            Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .areas(content);

        let selected_style = Style::new().fg(Color::Red);
        let default_style = Style::new().fg(Color::White);

        Block::bordered()
            .title("History")
            .style(match self.selected {
                OverviewArea::History => selected_style,
                _ => default_style,
            })
            .render(left, buf);

        //self.url.render_ref(input, buf);
        Block::bordered()
            .title("URL")
            .style(match self.selected {
                OverviewArea::URL => selected_style,
                _ => default_style,
            })
            .render(input, buf);

        Block::bordered()
            .title("Request")
            .style(match self.selected {
                OverviewArea::Request => selected_style,
                _ => default_style,
            })
            .render(request, buf);

        Block::bordered()
            .title("Response")
            .style(match self.selected {
                OverviewArea::Response => selected_style,
                _ => default_style,
            })
            .render(response, buf);

        Paragraph::new("Use ← ↑ → ↓ to navigate, <ENTER> to edit")
            .centered()
            .render(footer, buf);
    }
}

pub struct History {}
