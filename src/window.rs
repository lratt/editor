use std::io::Stdout;
use std::{io::Write, rc::Rc};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;
use crate::result::Result;

#[derive(Debug)]
pub struct Window {
    buffer: Rc<Buffer>,
    width: u16,
    height: u16,
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
    pub fn new(buffer: Rc<Buffer>, width: u16, height: u16) -> Window {
        Window {
            buffer,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn render(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        for (i, line) in (0..self.height).zip(
            self.buffer
                .lines
                .iter()
                .skip(self.offset_y)
                .take(self.height.into()),
        ) {
            queue!(stdout, MoveTo(0, i), Clear(ClearType::CurrentLine))?;
            let line = if line.len() > self.offset_x + usize::from(self.width) {
                &line[self.offset_x..self.offset_x + usize::from(self.width)]
            } else if line.len() > self.offset_x {
                &line[self.offset_x..]
            } else {
                ""
            };
            queue!(stdout, Print(line))?;
        }

        queue!(stdout, MoveTo(self.cursor_x, self.cursor_y))?;
        stdout.flush()?;

        Ok(())
    }

    fn adjust_column(&mut self) -> Result<()> {
        let line_index = self.offset_y + usize::from(self.cursor_y);
        let line = &self.buffer.lines[line_index];

        if self.offset_x + usize::from(self.cursor_x) > line.len() {
            if line.len() > usize::from(self.width) {
                self.cursor_x = self.width;
                self.offset_x = line.len() - usize::from(self.width);
            } else {
                self.cursor_x = line.len().try_into()?;
                self.offset_x = 0;
            }
        }

        Ok(())
    }

    pub fn move_cursor(&mut self, stdout: &mut Stdout, direction: Direction) -> Result<()> {
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
                if usize::from(self.cursor_y) < line_count - 1 && self.cursor_y < self.height - 1 {
                    self.cursor_y += 1;
                } else if self.offset_y + usize::from(self.cursor_y) < line_count - 1 {
                    self.offset_y += 1;
                }
            }
            Direction::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.offset_x > 0 {
                    self.offset_x -= 1;
                } else if self.cursor_x == 0 && usize::from(self.cursor_y) + self.offset_y > 0 {
                    self.cursor_y -= 1;

                    let line = &self.buffer.lines[self.offset_y + usize::from(self.cursor_y)];
                    if line.len() > usize::from(self.width) {
                        self.offset_x = line.len() - usize::from(self.width);
                        self.cursor_x = self.width;
                    } else {
                        self.offset_x = 0;
                        self.cursor_x = line.len().try_into()?;
                    }
                }
            }
            Direction::Right => {
                let current_line = &self.buffer.lines[self.offset_y + usize::from(self.cursor_y)];

                if usize::from(self.cursor_x) + self.offset_x == current_line.len() {
                    self.cursor_x = 0;
                    self.offset_x = 0;
                    self.cursor_y += 1;
                } else if self.cursor_x < self.width - 1
                    && usize::from(self.cursor_x) < current_line.len()
                {
                    self.cursor_x += 1;
                } else if self.offset_x + usize::from(self.cursor_x) < current_line.len() {
                    self.offset_x += 1;
                }
            }
        }

        self.adjust_column()?;

        queue!(stdout, MoveTo(self.cursor_x, self.cursor_y))?;

        Ok(())
    }
}
