/// A buffer represents the contents of a file as a vec of lines
pub struct Buffer {
    pub lines: Vec<String>,
}

impl From<String> for Buffer {
    fn from(s: String) -> Buffer {
        Buffer {
            lines: s.lines().map(|s| s.to_string()).collect(),
        }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(lines: Vec<String>) -> Buffer {
        Buffer { lines }
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut b = Buffer { lines: Vec::new() };
        b.lines.push("".to_string());
        b
    }

    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn line_at(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn char_at(&self, line: usize, char_index: usize) -> Option<char> {
        if let Some(ln) = self.line_at(line) {
            return ln.chars().skip(char_index).next();
        }
        None
    }

    pub fn char_size_at(&self, line: usize, char_index: usize) -> Option<usize> {
        if let Some(c) = self.char_at(line, char_index) {
            return Some(c.len_utf8());
        }
        None
    }

    pub fn line_byte_length(&self, line: usize) -> Option<usize> {
        if let Some(line) = self.line_at(line) {
            return Some(line.chars().fold(0, |acc, c| acc + c.len_utf8()));
        }
        None
    }

    pub fn line_at_mut(&mut self, index: usize) -> Option<&mut String> {
        self.lines.get_mut(index)
    }

    pub fn insert_line_at(&mut self, index: usize, line: String) {
        if index > self.lines.len() {
            self.lines.push(line);
            return;
        }
        self.lines.insert(index, line);
    }

    pub fn remove_line_at(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn break_line_at(&mut self, line_index: usize, char_index: usize) {
        let line = self.lines.remove(line_index);
        let (left, right) = line.split_at(char_index);
        let mut new_left = left.to_string();

        // to do: is this really necessary?!
        //new_left.push('\n');

        self.lines.insert(line_index, new_left);
        self.lines.insert(line_index + 1, right.to_string());
    }

    pub(crate) fn line_char_length(&self, cursor_line: usize) -> Option<usize> {
        if let Some(line) = self.line_at(cursor_line) {
            return Some(line.chars().fold(0, |acc, c| acc + c.len_utf8()));
        }
        None
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_construct_buffer() {
        let b = Buffer::new();
        assert_eq!(b.num_lines(), 1);
    }

    #[test]
    pub fn can_construct_buffer_from_string() {
        let b = Buffer::from("a\nb\nc".to_string());
        assert_eq!(b.num_lines(), 3);
    }

    #[test]
    pub fn can_construct_buffer_from_vec() {
        let b = Buffer::from(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(b.num_lines(), 3);
    }

    #[test]
    pub fn can_get_line_at() {
        let b = Buffer::from("a\nb\nc".to_string());
        assert_eq!(b.line_at(0), Some(&"a".to_string()));
        assert_eq!(b.line_at(1), Some(&"b".to_string()));
        assert_eq!(b.line_at(2), Some(&"c".to_string()));
    }

    #[test]
    pub fn non_existing_line_yields_none() {
        let b = Buffer::from("a\nb\nc".to_string());
        assert_eq!(b.line_at(3), None);
    }

    #[test]
    pub fn can_insert_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.insert_line_at(1, "d".to_string());
        assert_eq!(b.line_at(0), Some(&"a".to_string()));
        assert_eq!(b.line_at(1), Some(&"d".to_string()));
        assert_eq!(b.line_at(2), Some(&"b".to_string()));
        assert_eq!(b.line_at(3), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_remove_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.remove_line_at(1);
        assert_eq!(b.line_at(0), Some(&"a".to_string()));
        assert_eq!(b.line_at(1), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_change_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.line_at_mut(1).map(|l| *l = "d".to_string());
        assert_eq!(b.line_at(0), Some(&"a".to_string()));
        assert_eq!(b.line_at(1), Some(&"d".to_string()));
        assert_eq!(b.line_at(2), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_break_line_at() {
        let mut b = Buffer::from("a\nboo\nc".to_string());
        b.break_line_at(1, 1);
        assert_eq!(b.line_at(0), Some(&"a".to_string()));
        assert_eq!(b.line_at(1), Some(&"b\n".to_string()));
        assert_eq!(b.line_at(2), Some(&"oo".to_string()));
        assert_eq!(b.line_at(3), Some(&"c".to_string()));
    }
}
