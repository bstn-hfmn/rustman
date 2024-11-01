use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, WidgetRef},
};

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct TextInput {
    title: String,
    text: String,
    style: Style,
    pub cursor: usize,

    focused: bool,
}

impl TextInput {
    pub fn new(title: &str) -> Self {
        TextInput {
            style: Style::default(),
            title: String::from(title),
            text: String::new(),
            cursor: 0,
            focused: false,
        }
    }

    pub fn handle_key_event(
        &mut self,
        ev: KeyEvent,
    ) {
        match ev.code {
            KeyCode::Char(c) => self.append(c),
            KeyCode::Backspace => self.remove(),
            KeyCode::Left => match ev.modifiers {
                KeyModifiers::CONTROL => self.move_cursor(CursorDirection::Left, true),

                _ => self.move_cursor(CursorDirection::Left, false),
            },
            KeyCode::Right => match ev.modifiers {
                KeyModifiers::CONTROL => self.move_cursor(CursorDirection::Right, true),

                _ => self.move_cursor(CursorDirection::Right, false),
            },
            KeyCode::Delete => self.reset(),
            //KeyCode::Esc => self.toggle_mode(),
            _ => {}
        }
    }

    fn next_symbol_index(
        &self,
        direction: CursorDirection,
    ) -> usize {
        let index = match direction {
            CursorDirection::Left => self.text[..self.cursor]
                .char_indices()
                .rev()
                .find(|&(_, ch)| !ch.is_alphanumeric())
                .or(Some((0, '\0')))
                .map(|(i, _)| i)
                .unwrap(),
            CursorDirection::Right => self.text[self.cursor..]
                .char_indices()
                .skip(1)
                .find(|&(_, ch)| !ch.is_alphanumeric())
                .or(Some((self.text.chars().count(), '\0')))
                .map(|(i, _)| i + self.cursor)
                .unwrap(),

            _ => self.cursor,
        };

        index.clamp(0, self.text.chars().count())
    }

    pub fn move_cursor(
        &mut self,
        direction: CursorDirection,
        skip: bool,
    ) {
        let new: usize = match direction {
            CursorDirection::Right => match skip {
                false => self.cursor.saturating_add(1),
                true => self.next_symbol_index(direction),
            },
            CursorDirection::Left => match skip {
                false => self.cursor.saturating_sub(1),
                true => self.next_symbol_index(direction),
            },
            _ => self.cursor,
        };

        self.cursor = new.clamp(0, self.text.chars().count());
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.text.clear();
    }

    pub fn append(
        &mut self,
        c: char,
    ) {
        let index = self.index();
        self.text.insert(index, c);

        self.move_cursor(CursorDirection::Right, false);
    }

    pub fn remove(&mut self) {
        if self.cursor == 0 {
            return;
        }

        self.text = self
            .text
            .chars()
            .take(self.cursor - 1)
            .chain(self.text.chars().skip(self.cursor))
            .collect();

        self.move_cursor(CursorDirection::Left, false);
    }

    pub fn index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.text.len())
    }

    pub fn style(
        &mut self,
        s: Style,
    ) {
        self.style = s;
    }
}

impl Widget for TextInput {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for TextInput {
    fn render_ref(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let field = Paragraph::new(self.text.clone())
            .style(match self.focused {
                true => Style::default().fg(Color::Yellow),
                false => Style::default(),
            })
            .block(
                Block::default()
                    .borders(
                        Borders::TOP | Borders::LEFT | Borders::RIGHT | Borders::BOTTOM,
                    )
                    .title(self.title.clone())
                    .style(self.style),
            );

        field.render(area, buf);
    }
}
