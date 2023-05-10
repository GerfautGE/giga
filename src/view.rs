use crate::file::File;

/// The View struct represents the actual portion of the File being displayed.
pub struct View {
    /// The file being displayed
    file: File,
    /// The line number of the first line being displayed
    start_line: usize,
    /// The column number of the first column being displayed
    start_col: usize,
    /// The number of lines being displayed
    pub height: usize,
    /// The number of columns being displayed
    pub width: usize,
    /// The position of the cursor in the view
    cursor: (usize, usize),
}

impl View {
    /// Create a new View
    pub fn new(file: File, height: usize, width: usize) -> Self {
        Self {
            file,
            start_line: 0,
            start_col: 0,
            height,
            width,
            cursor: (0, 0),
        }
    }

    pub fn resize(&mut self, height: usize, width: usize) {
        self.height = height;
        self.width = width;
    }

    pub fn get_line(&self, index: usize) -> String {
        let line = self
            .file
            .get_line(index + self.start_line)
            .unwrap_or_default();
        let start = self.start_col.min(line.len());
        let end = (self.start_col + self.width).min(line.len());
        String::from_utf8_lossy(&line[start..end]).to_string()
    }

    pub fn navigate(&mut self, dx: isize, dy: isize) {
        self.cursor.0 = (self.cursor.0 as isize + dx).max(0) as usize;
        self.cursor.1 = (self.cursor.1 as isize + dy).max(0) as usize;
    }
}

impl ToString for View {
    fn to_string(&self) -> String {
        let bottom = self
            .height
            .min(self.file.len().saturating_sub(self.start_line));

        (0..bottom)
            .map(|i| self.get_line(i))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_new() {
        let view = View::new(File::new(), 10, 10);
        assert_eq!(view.start_line, 0);
        assert_eq!(view.start_col, 0);
        assert_eq!(view.height, 10);
        assert_eq!(view.width, 10);
    }

    #[test]
    fn view_to_string() {
        let view = View::new(File::from_bytes(b"Hello, World !\n"), 1, 10);
        assert_eq!(view.to_string(), "Hello, Wor");
    }

    #[test]
    fn view_resize() {
        let mut view = View::new(File::new(), 10, 10);
        view.resize(20, 20);
        assert_eq!(view.height, 20);
        assert_eq!(view.width, 20);
    }

    #[test]
    fn view_get_line() {
        let view = View::new(File::from_bytes(b"Hello, World !\n"), 1, 10);
        assert_eq!(view.get_line(0), "Hello, Wor");
    }
}
