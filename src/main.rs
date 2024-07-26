#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use std::rc::Rc;

pub mod buffer;
pub mod editor;
pub mod result;
pub mod window;

fn main() -> result::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let program = &args[0];

    if args.len() < 2 {
        println!("Usage: {program} <file>");
        std::process::exit(1);
    }

    let file = &args[1];

    let buffer = Rc::new(buffer::Buffer::open(file)?);
    let mut editor = editor::Editor::new(buffer);
    editor.run()?;

    Ok(())
}
