use crate::buffer::Buffer;

#[derive(Default)]
pub struct BufferEntry {
    pub name: String,
    pub buffer: Buffer,
    pub cursor_line: usize,
    // pub cursor_byte_position: usize,
    pub cursor_render_position: usize,
    pub modified: bool,
    pub scroll_offset: usize,

    pub selection_start: Option<(usize, usize)>, // line + char
    pub selection_end: Option<(usize, usize)>,   // line + char
}

fn graphemeindex_to_byte_pos(data: &str, index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    let indices: Vec<(usize, char)> = data.char_indices().collect();
    if let Some((index, _)) = indices.iter().skip(index - 1).next() {
        return *index;
    }
    0
}

impl BufferEntry {
    pub fn char_size_at_cursor(&self) -> Option<usize> {
        let res = self
            .buffer
            .char_size_at(self.cursor_line, self.cursor_render_position)
            .unwrap_or(0);

        Some(res)
    }

    pub fn skip_word_forward(&mut self) {
        let current_line = self.buffer.line_at(self.cursor_line).unwrap();

        // start position is the next non whitespace character:
        let mut pos = self.cursor_render_position;
        while pos < current_line.len() {
            if !current_line.iter().nth(pos).unwrap().is_whitespace() {
                break;
            }
            pos += 1;
        }
        self.cursor_render_position = pos;

        let mut pos = self.cursor_render_position;
        while pos < current_line.len() {
            if current_line.iter().nth(pos).unwrap().is_whitespace() {
                break;
            }
            pos += 1;
        }
        self.cursor_render_position = pos;
    }

    pub fn skip_word_backward(&mut self) {
        let current_line = self.buffer.line_at(self.cursor_line).unwrap();

        // start position is the next non whitespace character:
        let mut pos = self.cursor_render_position;
        while pos > 0 {
            if !current_line.iter().nth(pos - 1).unwrap().is_whitespace() {
                break;
            }
            pos -= 1;
        }
        self.cursor_render_position = pos;

        let mut pos = self.cursor_render_position;
        while pos > 0 {
            if current_line.iter().nth(pos - 1).unwrap().is_whitespace() {
                break;
            }
            pos -= 1;
        }
        self.cursor_render_position = pos;
    }

    pub fn goto_line_start(&mut self) {
        self.cursor_render_position = 0;
        // self.cursor_byte_position = 0;
    }

    pub fn goto_line_end(&mut self) {
        if let Some(line) = self.buffer.line_at(self.cursor_line) {
            self.cursor_render_position = self.buffer.line_char_length(self.cursor_line).unwrap();
            // self.cursor_byte_position = self.buffer.line_byte_length(self.cursor_line).unwrap();
        }
    }

    pub fn char_size_before_cursor(&self) -> Option<usize> {
        if self.cursor_render_position == 0 {
            return Some(0);
        }
        self.buffer
            .char_size_at(self.cursor_line, self.cursor_render_position - 1)
    }

    pub fn add_character(&mut self, c: char) {
        let current_line = self.buffer.line_at_mut(self.cursor_line);
        if let Some(line) = current_line {
            line.insert(self.cursor_render_position, c);
            // line.insert(self.cursor_byte_position, c);
            // self.cursor_byte_position += c.len_utf8();
            self.cursor_render_position += 1;
        }
        self.modified = true;
    }

    /// Remove the character before the current cursor position.
    ///
    /// If the cursor is at the beginning of a line, this will remove the line and
    /// move the cursor to the previous line.
    pub fn remove_character(&mut self) {
        // let char_size = self.char_size_before_cursor().unwrap();
        let current_line = self.buffer.line_at_mut(self.cursor_line);

        if let Some(line) = current_line {
            if self.cursor_render_position > 0 {
                line.remove(self.cursor_render_position - 1);
                self.cursor_render_position -= 1;
            } else {
                if self.buffer.num_lines() >= 1 {
                    self.buffer.remove_line_at(self.cursor_line);
                    self.cursor_line = self.cursor_line.saturating_sub(1);
                    if let Some(line) = self.buffer.line_at(self.cursor_line) {
                        self.cursor_render_position = line.len();
                    } else {
                        self.cursor_line = 0;
                    }
                }
            }
        }
        self.modified = true;
    }

    /// Insert a new line at the current position and move the cursor to the next line
    ///
    /// This will insert a new line at the current position and move the cursor to the beginning of the next line.
    /// If the current line is the last line, a new line will be inserted at the end of the buffer.
    /// If there are no lines in the buffer, a new line will be inserted at position 0.
    pub fn new_line(&mut self) {
        self.buffer
            .break_line_at(self.cursor_line, self.cursor_render_position);
        self.cursor_line += 1;
        //self.cursor_byte_position = 0;
        self.cursor_render_position = 0;
        self.modified = true;
    }

    /// Move the cursor up one line.
    ///
    /// If the cursor is already at the first line, this does nothing.
    ///
    /// This implementation is somewhat stupid and will always move to the end of
    /// the line. Ideally, this would move to the closest grapheme given the
    /// previous cursor position.
    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            if self.buffer.line_char_length(self.cursor_line).unwrap() < self.cursor_render_position
            {
                self.goto_line_end();
            }
        }
    }

    /// Move the cursor down one line.
    ///
    /// If the cursor is already at the last line, this does nothing.
    pub fn move_cursor_down(&mut self) {
        if self.cursor_line < self.buffer.num_lines() - 1 {
            self.cursor_line += 1;
            if self.buffer.line_char_length(self.cursor_line).unwrap() < self.cursor_render_position
            {
                self.goto_line_end();
            }
        }
    }

    /// Move the cursor one position to the left.
    ///
    /// If the cursor is not at the start of the line, this function moves the cursor
    /// left by one grapheme and adjusts the byte position accordingly.

    pub fn move_cursor_left(&mut self) {
        if self.cursor_render_position > 0 {
            self.cursor_render_position -= 1;
        }
    }

    /// Move the cursor one position to the right.
    ///
    /// If the cursor is not at the end of the line, this function moves the cursor
    /// right by one grapheme and adjusts the byte position accordingly.
    pub fn move_cursor_right(&mut self) {
        if self.cursor_render_position < self.buffer.line_char_length(self.cursor_line).unwrap() {
            self.cursor_render_position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Line;

    use super::*;

    fn assert_line_equals(l1: &Line, l2: &str) {
        assert_eq!(l1.iter().collect::<String>().as_str(), l2);
    }

    fn inject_string(b: &mut BufferEntry, s: &str) {
        for c in s.chars() {
            b.add_character(c);
        }
    }

    #[test]
    pub fn inject_backspace_modifies_buffer() {
        let mut b = BufferEntry::default();
        b.add_character('a');
        b.remove_character();

        let ln = b.buffer.line_at(0).unwrap();
        assert_line_equals(ln, "");
    }

    #[test]
    pub fn inject_char_modifies_buffer() {
        let mut b = BufferEntry::default();
        b.add_character('a');

        let ln = b.buffer.line_at(0).unwrap();
        assert_line_equals(ln, "a");
    }

    #[test]
    pub fn inject_enter_modifies_buffer() {
        let mut b = BufferEntry::default();
        b.add_character('a');
        b.new_line();

        let ln = b.buffer.line_at(0).unwrap();
        assert_line_equals(ln, "a");

        let ln = b.buffer.line_at(1).unwrap();
        assert_line_equals(ln, "");
    }

    #[test]
    pub fn backspace_on_empty_line_removes_line() {
        let mut b = BufferEntry::default();
        assert_eq!(b.buffer.num_lines(), 1);
        b.remove_character();
        assert_eq!(b.buffer.num_lines(), 0);
    }

    #[test]
    pub fn backspace_on_empty_buffer_does_not_crash() {
        let mut b = BufferEntry::default();
        assert_eq!(b.buffer.num_lines(), 1);
        b.remove_character();
        b.remove_character();
        b.remove_character();
        assert_eq!(b.buffer.num_lines(), 0);
    }

    #[test]
    pub fn can_skip_word_forward() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.goto_line_start();
        b.skip_word_forward();
        assert_eq!(b.cursor_render_position, 4);
        b.skip_word_forward();
        assert_eq!(b.cursor_render_position, 8);
        b.skip_word_forward();
        assert_eq!(b.cursor_render_position, 12);
        b.skip_word_forward();
        assert_eq!(b.cursor_render_position, 12);
    }

    #[test]
    pub fn can_skip_word_backward() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.goto_line_end();
        b.skip_word_backward();
        assert_eq!(b.cursor_render_position, 9);
        b.skip_word_backward();
        assert_eq!(b.cursor_render_position, 5);
        b.skip_word_backward();
        assert_eq!(b.cursor_render_position, 0);
        b.skip_word_backward();
        assert_eq!(b.cursor_render_position, 0);
    }

    #[test]
    pub fn move_cursor_up_into_empty_line() {
        let mut b = BufferEntry::default();
        b.new_line();
        inject_string(&mut b, "argh foo bar");
        b.move_cursor_up();
        assert_eq!(b.cursor_render_position, 0);
    }
}
