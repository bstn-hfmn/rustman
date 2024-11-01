use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
};
use ratatui::{
    layout::{Constraint, Layout, Position},
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};
use std::io::stdout;

use crate::ui::widgets::input::CursorMoveDirection;
use crate::ui::widgets::TextField;

enum InputMode {
    Edit,
    Navigate,
}

pub struct App {
    running: bool,
    mode: InputMode,

    url_field: TextField,
}

impl App {
    pub fn new() -> Self {
        App {
            running: false,
            mode: InputMode::Navigate,
            url_field: TextField::new("URL"),
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
        let ar = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

        let [header, _, view_tabs, detail_tabs, main, footer] =
            Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .areas(frame.area());

        match self.mode {
            InputMode::Navigate => {
                crossterm::execute!(stdout(), cursor::Hide)?;
                self.url_field.unfocus();
            }
            InputMode::Edit => {
                crossterm::execute!(stdout(), cursor::Show)?;

                self.url_field.focus();
                frame.set_cursor_position(Position::new(
                    header.x + self.url_field.cursor as u16 + 1,
                    header.y + 1,
                ));
            }
        }

        frame.render_widget(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP),
            ar[2],
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP),
            ar[3],
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP),
            ar[4],
        );
        frame.render_widget(Block::default(), ar[5]);

        frame.render_widget(self.url_field.clone(), ar[0]);
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(ev) = event::read()? {
            match self.mode {
                InputMode::Navigate if ev.kind == KeyEventKind::Press => {
                    match ev.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        KeyCode::Char('e') => self.toggle_mode(),
                        _ => {}
                    }
                }

                InputMode::Edit if ev.kind == KeyEventKind::Press => {
                    match ev.code {
                        KeyCode::Char(c) => self.url_field.append(c),
                        KeyCode::Backspace => self.url_field.remove(),
                        KeyCode::Left => match ev.modifiers {
                            KeyModifiers::CONTROL => self
                                .url_field
                                .move_cursor(CursorMoveDirection::Left, true),

                            _ => self
                                .url_field
                                .move_cursor(CursorMoveDirection::Left, false),
                        },
                        KeyCode::Right => match ev.modifiers {
                            KeyModifiers::CONTROL => self
                                .url_field
                                .move_cursor(CursorMoveDirection::Right, true),

                            _ => self
                                .url_field
                                .move_cursor(CursorMoveDirection::Right, false),
                        },
                        KeyCode::Delete => self.url_field.reset(),
                        KeyCode::Esc => self.toggle_mode(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            InputMode::Edit => InputMode::Navigate,
            InputMode::Navigate => InputMode::Edit,
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
