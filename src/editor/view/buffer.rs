use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(String::from(value));
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool{
        self.lines.is_empty()
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        if let Some(line_content) = self.lines.get_mut(line) {
            line_content.insert(col, c);
        }
    }

    pub fn delete_char(&mut self, line: usize, col: usize) {
        if let Some(line_content) = self.lines.get_mut(line) {
            if col < line_content.len() {
                line_content.remove(col);
            }
        }
    }

    pub fn save(&self, file_name: &str) -> Result<(), Error> {
        let contents = self.lines.join("\n");
        std::fs::write(file_name, contents)
    }
}