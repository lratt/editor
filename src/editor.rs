use std::io::{self};
use std::path::Path;
use std::rc::Rc;

use crate::buffer::Buffer;
use crate::result::Result;
use crate::window::{self, Window};
use crossterm::event::{Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

#[derive(Debug)]
pub struct Editor {
    window: Window,
    stdout: io::Stdout,
}

impl Editor {
    #[must_use]
    pub fn new(buffer: Rc<Buffer>) -> Editor {
        let window = Window::new(buffer);

        Editor {
            window,
            stdout: std::io::stdout(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;

        self.window.render(&mut self.stdout)?;

        loop {
            let event = crossterm::event::read()?;

            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Down => {
                        self.window
                            .move_cursor(&mut self.stdout, window::Direction::Down)?;
                    }
                    KeyCode::Up => {
                        self.window
                            .move_cursor(&mut self.stdout, window::Direction::Up)?;
                    }
                    KeyCode::Left => {
                        self.window
                            .move_cursor(&mut self.stdout, window::Direction::Left)?;
                    }
                    KeyCode::Right => {
                        self.window
                            .move_cursor(&mut self.stdout, window::Direction::Right)?;
                    }
                    _ => {}
                },
                _ => todo!(),
            }

            self.window.render(&mut self.stdout)?;
        }

        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().ok();
        execute!(self.stdout, LeaveAlternateScreen).ok();
    }
}
