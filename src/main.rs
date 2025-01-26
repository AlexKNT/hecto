#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {   
    editor::default().run();
}