mod documents;
mod editor;
mod row;
mod terminal;

pub use documents::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
