pub type Line = Vec<char>;

/// A buffer represents the contents of a file as a vec of lines

pub struct Buffer {
    pub lines: Vec<Line>,
}

fn string_to_line(s: &str) -> Line {
    s.chars().collect()
}

impl From<String> for Buffer {
    fn from(s: String) -> Buffer {
        let lines = s.lines().map(|s| string_to_line(s)).collect::<Vec<Line>>();
        Buffer { lines: lines }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(lines: Vec<String>) -> Buffer {
        Buffer {
            lines: lines.iter().map(|s| string_to_line(s)).collect(),
        }
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut b = Buffer { lines: Vec::new() };
        b.lines.push(string_to_line(""));
        b
    }

    pub fn empty_buffer() -> Buffer {
        Buffer { lines: Vec::new() }
    }

    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn line_at(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }

    pub fn line_at_mut(&mut self, index: usize) -> Option<&mut Line> {
        self.lines.get_mut(index)
    }

    pub fn insert_line_at(&mut self, index: usize, line: Line) {
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
        let new_left = left.to_vec();

        self.lines.insert(line_index, new_left);
        self.lines.insert(line_index + 1, right.to_vec());
    }

    pub fn line_char_length(&self, cursor_line: usize) -> Option<usize> {
        if let Some(line) = self.line_at(cursor_line) {
            return Some(line.len());
        }
        None
    }

    pub fn merge_lines(&mut self, cursor_line_1: usize, cursor_line_2: usize) {
        let mut merged_line = self.lines[cursor_line_1].clone();
        merged_line.extend(self.lines[cursor_line_2].clone());
        self.lines.remove(cursor_line_1);
        self.lines.remove(cursor_line_1); // we remove line index 1 twice, since the original second line is now at index 1
        self.lines.insert(cursor_line_1, merged_line);
    }

    pub(crate) fn add_line(&mut self, line: &str) {
        self.lines.push(line.chars().collect());
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

    fn assert_line_equals(l1: &Line, l2: &str) {
        assert_eq!(l1.iter().collect::<String>().as_str(), l2);
    }

    fn assert_line_equals_optional(l1: Option<&Line>, l2: Option<&str>) {
        if let Some(l2) = l2 {
            assert_line_equals(l1.unwrap(), l2);
        } else {
            assert!(false);
        }
    }

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
        assert_line_equals_optional(b.line_at(0), Some(&"a".to_string()));
        assert_line_equals_optional(b.line_at(1), Some(&"b".to_string()));
        assert_line_equals_optional(b.line_at(2), Some(&"c".to_string()));
    }

    #[test]
    pub fn non_existing_line_yields_none() {
        let b = Buffer::from("a\nb\nc".to_string());
        assert_eq!(b.line_at(3), None);
    }

    #[test]
    pub fn can_insert_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.insert_line_at(1, string_to_line("d"));
        assert_line_equals_optional(b.line_at(0), Some(&"a".to_string()));
        assert_line_equals_optional(b.line_at(1), Some(&"d".to_string()));
        assert_line_equals_optional(b.line_at(2), Some(&"b".to_string()));
        assert_line_equals_optional(b.line_at(3), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_remove_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.remove_line_at(1);
        assert_line_equals_optional(b.line_at(0), Some(&"a".to_string()));
        assert_line_equals_optional(b.line_at(1), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_change_line_at() {
        let mut b = Buffer::from("a\nb\nc".to_string());
        b.line_at_mut(1).map(|l| *l = string_to_line("d"));
        assert_line_equals_optional(b.line_at(0), Some(&"a".to_string()));
        assert_line_equals_optional(b.line_at(1), Some(&"d".to_string()));
        assert_line_equals_optional(b.line_at(2), Some(&"c".to_string()));
    }

    #[test]
    pub fn can_break_line_at() {
        let mut b = Buffer::from("a\nboo\nc".to_string());
        b.break_line_at(1, 1);
        assert_line_equals_optional(b.line_at(0), Some(&"a".to_string()));
        assert_line_equals_optional(b.line_at(1), Some(&"b".to_string()));
        assert_line_equals_optional(b.line_at(2), Some(&"oo".to_string()));
        assert_line_equals_optional(b.line_at(3), Some(&"c".to_string()));
    }
}
