use super::terminal::{Size, Terminal};
mod buffer;
use buffer::Buffer;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    offset_y: usize,  // Add this line
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self, offset_y: usize) {
        self.needs_redraw = true; 
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            let buffer_row = current_row + offset_y;  // Use the passed offset_y
            if let Some(line) = self.buffer.lines.get(buffer_row) {
                let truncated_line = if line.len() > width { &line[..width] } else { line };
                Self::render_line(current_row, truncated_line);
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        let actual_line = line + self.offset_y;  // Adjust line index for offset
        self.buffer.insert_char(actual_line, col, c);
        self.needs_redraw = true;
    }

    pub fn delete_char(&mut self, line: usize, col: usize) {
        let actual_line = line + self.offset_y;  // Adjust line index for offset
        self.buffer.delete_char(actual_line, col);
        self.needs_redraw = true;
    }

    pub fn save_buffer(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.buffer.save(file_name)
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            offset_y: 0,  // Initialize offset_y
        }
    }
}