use crate::{file::File, tui::Tui, view::View};
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
}

impl Editor {
    pub fn new(file_name: Option<&str>) -> Self {
        Self {
            file_name: file_name.map(|s| s.to_string()),
            view: View::new(File::new(), 10, 20),
            tui: Tui::new(),
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
        })
    }

    pub fn run(&mut self) {
        // set view size
        let (width, height) = self.tui.get_term_size();
        // height - 1 to leave space for the status bar
        self.view.resize((height - 1) as usize, width as usize);

        // // Spawn a thread to asynchronously read input from stdin
        // let queue = self.command_queue.clone();
        // let _input_thread = std::thread::spawn(|| {
        //     Self::process_input(queue);
        // });

        // draw initial view
        self.tui.clear();
        self.tui.draw_view(&self.view, &self.file_name);

        let stdin = std::io::stdin().keys();

        for c in stdin {
            match c.unwrap_or(termion::event::Key::Char(char::from('j'))) {
                termion::event::Key::Char('q') => {
                    self.tui.clear();
                    break;
                }
                termion::event::Key::Char('j') => {
                    self.view.navigate(0, 1);
                    self.tui.draw_view(&self.view, &self.file_name);
                }
                termion::event::Key::Char('k') => {
                    self.view.navigate(0, -1);
                    self.tui.draw_view(&self.view, &self.file_name);
                }
                termion::event::Key::Char('h') => {
                    self.view.navigate(-1, 0);
                    self.tui.draw_view(&self.view, &self.file_name);
                }
                termion::event::Key::Char('l') => {
                    self.view.navigate(1, 0);
                    self.tui.draw_view(&self.view, &self.file_name);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_new_empty() {
        let editor = Editor::new(Some("filename"));
        assert_eq!(editor.view.to_string(), "");
        assert_eq!(editor.file_name, Some("filename".to_string()));
    }

    #[test]
    fn editor_open() {
        let path = "tests/sample.txt";
        let editor = Editor::open(path);
        assert!(editor.is_ok());

        let editor = editor.unwrap();

        let expected = "Hello, World !\n";
        assert_eq!(editor.view.to_string(), expected);
        assert_eq!(editor.file_name, Some("tests/sample.txt".to_string()));
    }

    #[test]
    fn editor_open_error() {
        let path = "tests/does_not_exist.txt";
        let editor = Editor::open(path);
        assert!(editor.is_err());
    }
}
