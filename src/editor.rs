use std::process::exit;

use crate::{command::Command, file::File, tui::Tui, view::View};
use termion::input::TermRead;

/// Editor structure
/// represents the state of the program
pub struct Editor {
    /// The name of the file being edited
    file_name: Option<String>,
    /// The current view of the file
    view: View,
    /// The Tui responsible for drawing the editor
    tui: Tui,
    /// The mode of the editor
    mode: Mode,
}

/// Mode of the editor
pub enum Mode {
    /// Normal mode
    Normal,
    /// Insert mode
    Insert,
}

impl Editor {
    pub fn new(file_name: Option<&str>) -> Self {
        Self {
            file_name: file_name.map(|s| s.to_string()),
            view: View::new(File::new(), 10, 20),
            tui: Tui::new(),
            mode: Mode::Normal,
        }
    }

    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read(path)?;
        let content = File::from_bytes(&content);
        let view = View::new(content, 10, 20);

        Ok(Self {
            file_name: Some(path.to_string()),
            view,
            tui: Tui::new(),
            mode: Mode::Normal,
        })
    }

    fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::Quit => {
                self.tui.cleanup();
                exit(0);
            }
            Command::Move(x, y) => self.view.navigate(x, y),
            Command::Save => {
                if let Some(path) = &self.file_name {
                    let content = self.view.dump_file();
                    std::fs::write(path.clone() + ".tmp", content).unwrap_or_default();
                    std::fs::rename(path.clone() + ".tmp", path).unwrap_or_default();
                }
            }
            Command::ToggleMode => {
                self.mode = match self.mode {
                    Mode::Normal => Mode::Insert,
                    Mode::Insert => Mode::Normal,
                }
            }
            Command::Insert(c) => self.view.insert(c),
            Command::InsertNewLine => self.view.insert_new_line(),
            Command::Delete => self.view.delete(),
        }
    }

    pub fn run(&mut self) {
        // set view size
        let (width, height) = self.tui.get_term_size();
        // height - 1 to leave space for the status bar
        self.view.resize((height - 1) as usize, width as usize);

        // draw initial view
        self.tui.clear();
        self.tui.draw_view(&self.view, &self.file_name, &self.mode);

        let stdin = std::io::stdin().keys();

        for c in stdin {
            if let Ok(c) = c {
                if let Ok(cmd) = Command::parse(c, &self.mode) {
                    self.execute(cmd);
                    self.tui.draw_view(&self.view, &self.file_name, &self.mode)
                }
            }
        }
    }
}
