use crossterm::{
    cursor::{self},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
};
use ratatui::{
    layout::{Constraint, Layout, Position},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io::stdout;

use crate::ui::widgets::{CursorDirection, TextInput};
use crate::ui::Overview;

enum UIState {
    EditURL,
    EditRequest,

    NavigateHistory,
    NavigateOverview,
    NavigateResponse,
}

pub struct App {
    running: bool,
    state: UIState,

    overview: Overview,
}

impl App {
    pub fn new() -> Self {
        App {
            running: false,
            overview: Overview::new(),

            state: UIState::NavigateOverview,
        }
    }

    pub fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> std::io::Result<()> {
        self.running = true;

        while self.is_running() {
            terminal.draw(|frame| {
                self.render(frame).expect("Failed to render frame.");
            })?;

            self.handle_events()?;
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn render(
        &mut self,
        frame: &mut Frame,
    ) -> std::io::Result<()> {
        match self.state {
            UIState::NavigateHistory => {}
            UIState::NavigateResponse => {}
            UIState::NavigateOverview => {
                crossterm::execute!(stdout(), cursor::Hide)?;

                //self.url.unfocus();
            }
            UIState::EditURL | UIState::EditRequest => {
                crossterm::execute!(stdout(), cursor::Show)?;

                //self.url.focus();
                //frame.set_cursor_position(Position::new(
                //    input.x + self.url.cursor as u16 + 1,
                //    input.y + 1,
                //));
            }
        }

        frame.render_widget(&self.overview, frame.area());
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(ev) = event::read()? {
            match self.state {
                UIState::NavigateOverview if ev.kind == KeyEventKind::Press => {
                    match ev.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => self.overview.handle_key_event(ev),
                    }
                }

                UIState::EditURL if ev.kind == KeyEventKind::Press => match ev.code {
                    KeyCode::Esc => self.state = UIState::NavigateOverview,
                    _ => self.overview.url.handle_key_event(ev),
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
