use ratatui::widgets::Widget;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub enum CursorMoveDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct TextField {
    title: String,
    text: String,
    pub cursor: usize,

    focused: bool,
}

impl TextField {
    pub fn new(title: &str) -> Self {
        TextField {
            title: String::from(title),
            text: String::new(),
            cursor: 0,
            focused: false,
        }
    }

    fn next_symbol_index(
        &self,
        direction: CursorMoveDirection,
    ) -> usize {
        let index = match direction {
            CursorMoveDirection::Left => self.text[..self.cursor]
                .char_indices()
                .rev()
                .find(|&(_, ch)| !ch.is_alphanumeric())
                .or(Some((0, '\0')))
                .map(|(i, _)| i)
                .unwrap(),
            CursorMoveDirection::Right => self.text[self.cursor..]
                .char_indices()
                .skip(1)
                .find(|&(_, ch)| !ch.is_alphanumeric())
                .or(Some((self.text.chars().count(), '\0')))
                .map(|(i, _)| i + self.cursor)
                .unwrap(),
        };

        index.clamp(0, self.text.chars().count())
    }

    pub fn move_cursor(
        &mut self,
        direction: CursorMoveDirection,
        skip: bool,
    ) {
        let new: usize = match direction {
            CursorMoveDirection::Right => match skip {
                false => self.cursor.saturating_add(1),
                true => self.next_symbol_index(direction),
            },
            CursorMoveDirection::Left => match skip {
                false => self.cursor.saturating_sub(1),
                true => self.next_symbol_index(direction),
            },
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

        self.move_cursor(CursorMoveDirection::Right, false);
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

        self.move_cursor(CursorMoveDirection::Left, false);
    }

    pub fn index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.text.len())
    }
}

impl Widget for TextField {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let field = Paragraph::new(self.text)
            .style(match self.focused {
                true => Style::default().fg(Color::Yellow),
                false => Style::default(),
            })
            .block(
                Block::default()
                    .borders(
                        Borders::TOP
                            | Borders::LEFT
                            | Borders::RIGHT
                            | Borders::BOTTOM,
                    )
                    .title(self.title)
                    .style(Style::default().fg(Color::default())),
            );

        field.render(area, buf);
    }
}
