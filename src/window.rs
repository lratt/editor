use std::io::Stdout;
use std::{io::Write, rc::Rc};

use crossterm::queue;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor::MoveTo, terminal};

use crate::buffer::Buffer;
use crate::result::Result;

#[derive(Debug)]
pub struct Window {
    buffer: Rc<Buffer>,
    cursor_x: u16,
    cursor_y: u16,
    offset_x: usize,
    offset_y: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Window {
    #[must_use]
    pub fn new(buffer: Rc<Buffer>) -> Window {
        Window {
            buffer,
            cursor_x: 0,
            cursor_y: 0,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn render(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        let (_, row_count) = terminal::size()?;

        for (i, line) in (0..row_count).zip(
            self.buffer
                .lines
                .iter()
                .skip(self.offset_y)
                .take(row_count.into()),
        ) {
            queue!(stdout, MoveTo(0, i), Clear(ClearType::CurrentLine))?;
            stdout.write_all(line.as_bytes())?;
        }

        queue!(stdout, MoveTo(self.cursor_x, self.cursor_y))?;
        stdout.flush()?;

        Ok(())
    }

    pub fn move_cursor(&mut self, stdout: &mut Stdout, direction: Direction) -> Result<()> {
        let (_, row_count) = terminal::size()?;
        let line_count = self.buffer.lines.len();

        match direction {
            Direction::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                } else if self.offset_y > 0 {
                    self.offset_y -= 1;
                }
            }
            Direction::Down => {
                if usize::from(self.cursor_y) < line_count - 1 && self.cursor_y < row_count - 1 {
                    self.cursor_y += 1;
                } else if self.offset_y + usize::from(self.cursor_y) < line_count - 1 {
                    self.offset_y += 1;
                }
            }
            Direction::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            Direction::Right => self.cursor_x = self.cursor_x.saturating_add(1),
        }
        queue!(stdout, MoveTo(self.cursor_x, self.cursor_y))?;

        Ok(())
    }
}
