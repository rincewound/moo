use std::io::{BufRead, BufReader, Read};

use crate::buffer::Buffer;

#[derive(Default)]
pub struct BufferEntry {
    pub name: String,
    pub buffer: Buffer,
    pub cursor_line: usize,
    pub cursor_position: usize,
    pub modified: bool,
    pub scroll_offset: usize,

    pub selection_start: Option<(usize, usize)>, // line + char
    pub selection_end: Option<(usize, usize)>,   // line + char
}

impl BufferEntry {
    fn empty() -> BufferEntry {
        BufferEntry {
            name: String::new(),
            buffer: Buffer::empty_buffer(),
            cursor_line: 0,
            cursor_position: 0,
            modified: false,
            scroll_offset: 0,
            selection_start: None,
            selection_end: None,
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    pub fn skip_word_forward(&mut self) {
        let current_line = self.buffer.line_at(self.cursor_line).unwrap();

        // if the cursor is already at the end of the line, do nothing
        if self.cursor_position >= current_line.len() {
            if self.cursor_line <= self.buffer.num_lines() - 1 {
                self.cursor_line += 1;
                self.cursor_position = 0;
                return;
            }
        }

        // start position is the next non whitespace character:
        let mut pos = self.cursor_position;
        while pos < current_line.len() {
            if !current_line.iter().nth(pos).unwrap().is_whitespace() {
                break;
            }
            pos += 1;
        }
        self.cursor_position = pos;

        let mut pos = self.cursor_position;
        while pos < current_line.len() {
            if current_line.iter().nth(pos).unwrap().is_whitespace() {
                break;
            }
            pos += 1;
        }
        self.cursor_position = pos;
    }

    pub fn skip_word_backward(&mut self) {
        let current_line = self.buffer.line_at(self.cursor_line).unwrap();

        // if the cursor is already at the start of the line, do nothing
        if self.cursor_position == 0 {
            if self.cursor_line > 0 {
                self.cursor_line -= 1;
                self.cursor_position = self.buffer.line_char_length(self.cursor_line).unwrap();
                return;
            }
        }

        // start position is the next non whitespace character:
        let mut pos = self.cursor_position;
        while pos > 0 {
            if !current_line.iter().nth(pos - 1).unwrap().is_whitespace() {
                break;
            }
            pos -= 1;
        }
        self.cursor_position = pos;

        let mut pos = self.cursor_position;
        while pos > 0 {
            if current_line.iter().nth(pos - 1).unwrap().is_whitespace() {
                break;
            }
            pos -= 1;
        }
        self.cursor_position = pos;
    }

    pub fn goto_line_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn goto_line_end(&mut self) {
        if let Some(_) = self.buffer.line_at(self.cursor_line) {
            self.cursor_position = self.buffer.line_char_length(self.cursor_line).unwrap();
        }
    }

    pub fn add_character(&mut self, c: char) {
        let current_line = self.buffer.line_at_mut(self.cursor_line);
        if let Some(line) = current_line {
            line.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
        self.modified = true;
    }

    /// Remove the character before the current cursor position.
    ///
    /// If the cursor is at the beginning of a line, this will remove the line and
    /// move the cursor to the previous line.
    pub fn remove_character(&mut self, screen_height: u16) {
        // let char_size = self.char_size_before_cursor().unwrap();
        let current_line = self.buffer.line_at_mut(self.cursor_line);

        if let Some(line) = current_line {
            if self.cursor_position > 0 {
                line.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
            } else {
                // If the cursor is at the beginning of a line, remove the line and move the cursor to the previous line
                // also, copy any characters from the previous line to the current line
                if self.buffer.num_lines() > 1 {
                    let current_line_len = self.buffer.line_char_length(self.cursor_line).unwrap();

                    self.buffer
                        .merge_lines(self.cursor_line - 1, self.cursor_line);

                    self.cursor_line = self.cursor_line.saturating_sub(1);
                    if let Some(line) = self.buffer.line_at(self.cursor_line) {
                        self.cursor_position = line.len() - current_line_len;
                    } else {
                        self.cursor_line = 0;
                    }
                }
                // special case: this is the last line in the buffer, just remove it:
                else {
                    self.buffer.remove_line_at(self.cursor_line);
                    self.cursor_line = 0;
                    self.cursor_position = 0;
                }
            }
        }
        self.update_scroll_position(screen_height);
        self.modified = true;
    }

    /// Insert a new line at the current position and move the cursor to the next line
    ///
    /// This will insert a new line at the current position and move the cursor to the beginning of the next line.
    /// If the current line is the last line, a new line will be inserted at the end of the buffer.
    /// If there are no lines in the buffer, a new line will be inserted at position 0.
    pub fn new_line(&mut self, screen_height: u16) {
        self.buffer
            .break_line_at(self.cursor_line, self.cursor_position);
        self.cursor_line += 1;
        self.cursor_position = 0;
        self.update_scroll_position(screen_height);
        self.modified = true;
    }

    /// Move the cursor up one line.
    ///
    /// If the cursor is already at the first line, this does nothing.
    ///
    /// This implementation is somewhat stupid and will always move to the end of
    /// the line. Ideally, this would move to the closest grapheme given the
    /// previous cursor position.
    pub fn move_cursor_up(&mut self, screen_height: u16) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.update_scroll_position(screen_height);
        }
    }

    /// Move the cursor down one line.
    ///
    /// If the cursor is already at the last line, this does nothing.
    pub fn move_cursor_down(&mut self, screen_height: u16) {
        if self.cursor_line < self.buffer.num_lines() - 1 {
            self.cursor_line += 1;
            self.update_scroll_position(screen_height);
        }
    }

    pub fn move_cursor_page_down(&mut self, screen_height: u16) {
        self.cursor_line += screen_height as usize;
        self.update_scroll_position(screen_height);
    }

    pub fn move_cursor_page_up(&mut self, screen_height: u16) {
        self.cursor_line = self.cursor_line.saturating_sub(screen_height as usize);
        self.update_scroll_position(screen_height);
    }

    fn update_scroll_position(&mut self, screen_height: u16) {
        if self.cursor_line > self.buffer.num_lines().saturating_sub(1) {
            self.cursor_line = self.buffer.num_lines() - 1;
        }

        let on_screen_cursor_y = self.cursor_line as i32 - self.scroll_offset as i32;
        if on_screen_cursor_y > screen_height as i32 {
            self.scroll_offset = (on_screen_cursor_y - screen_height as i32) as usize;
        }
        let on_screen_cursor_y = self.cursor_line as i32 - self.scroll_offset as i32;
        if on_screen_cursor_y < 0 {
            self.scroll_offset = 0;
        }

        if let Some(pos) = self.buffer.line_char_length(self.cursor_line) {
            if self.cursor_position > pos {
                self.goto_line_end();
            }
        }
    }

    /// Move the cursor one position to the left.
    ///
    /// If the cursor is not at the start of the line, this function moves the cursor
    /// left by one grapheme and adjusts the byte position accordingly.

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
        if self.cursor_position == 0 && self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_position = self.buffer.line_char_length(self.cursor_line).unwrap();
        }
    }

    /// Move the cursor one position to the right.
    ///
    /// If the cursor is not at the end of the line, this function moves the cursor
    /// right by one grapheme and adjusts the byte position accordingly.
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.buffer.line_char_length(self.cursor_line).unwrap() {
            self.cursor_position += 1;
        } else {
            if self.cursor_line < self.buffer.num_lines() - 1 {
                self.cursor_position = 0;
                self.cursor_line += 1;
            }
        }
    }

    pub(crate) fn from_file(file_name: String) -> BufferEntry {
        let mut buffer = BufferEntry::empty();
        buffer.name = file_name.clone();

        //load data from file:
        let file = std::fs::File::open(file_name.clone()).unwrap();

        let reader = BufReader::new(file);
        for line in reader.lines() {
            buffer.buffer.add_line(line.unwrap().as_str());
        }

        buffer
    }

    pub fn extend_selection_to_cursor(&mut self) {
        if self.selection_start.is_none() {
            self.selection_start = Some((self.cursor_line, self.cursor_position));
        }
        self.selection_end = Some((self.cursor_line, self.cursor_position));
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
        b.new_line(40);
    }

    #[test]
    pub fn inject_backspace_modifies_buffer() {
        let mut b = BufferEntry::default();
        b.add_character('a');
        b.remove_character(0);

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
        b.new_line(0);

        let ln = b.buffer.line_at(0).unwrap();
        assert_line_equals(ln, "a");

        let ln = b.buffer.line_at(1).unwrap();
        assert_line_equals(ln, "");
    }

    #[test]
    pub fn backspace_on_empty_line_removes_line() {
        let mut b = BufferEntry::default();
        assert_eq!(b.buffer.num_lines(), 1);
        b.remove_character(0);
        assert_eq!(b.buffer.num_lines(), 0);
    }

    #[test]
    pub fn backspace_on_empty_buffer_does_not_crash() {
        let mut b = BufferEntry::default();
        assert_eq!(b.buffer.num_lines(), 1);
        b.remove_character(0);
        b.remove_character(0);
        b.remove_character(0);
        assert_eq!(b.buffer.num_lines(), 0);
    }

    #[test]
    pub fn can_skip_word_forward() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.cursor_line = 0;
        b.goto_line_start();
        b.skip_word_forward();
        assert_eq!(b.cursor_position, 4);
        b.skip_word_forward();
        assert_eq!(b.cursor_position, 8);
        b.skip_word_forward();
        assert_eq!(b.cursor_position, 12);
    }

    #[test]
    pub fn skip_forward_at_line_end_skips_to_next_line_start() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        inject_string(&mut b, "again");
        b.cursor_line = 0;
        b.cursor_position = 0;
        b.goto_line_end();
        b.skip_word_forward();
        assert_eq!(b.cursor_position, 0);
        assert_eq!(b.cursor_line, 1);
    }

    #[test]
    pub fn can_skip_word_backward() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.cursor_line = 0;
        b.goto_line_end();
        b.skip_word_backward();
        assert_eq!(b.cursor_position, 9);
        b.skip_word_backward();
        assert_eq!(b.cursor_position, 5);
        b.skip_word_backward();
        assert_eq!(b.cursor_position, 0);
        b.skip_word_backward();
        assert_eq!(b.cursor_position, 0);
    }

    #[test]
    pub fn skip_backward_at_line_start_skips_to_prev_line_end() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        inject_string(&mut b, "again");
        b.cursor_line = 1;
        b.cursor_position = 0;
        b.goto_line_start();
        b.skip_word_backward();
        assert_eq!(b.cursor_position, 12);
        assert_eq!(b.cursor_line, 0);
    }

    #[test]
    pub fn move_cursor_up_into_empty_line() {
        let mut b = BufferEntry::default();
        b.new_line(0);
        inject_string(&mut b, "argh foo bar");
        b.move_cursor_up(32);
        assert_eq!(b.cursor_position, 0);
    }

    #[test]
    pub fn move_cursor_right_skips_to_new_line() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.new_line(0);
        b.cursor_line = 0;
        b.goto_line_end();
        b.move_cursor_right();
        assert_eq!(b.cursor_position, 0);
        assert_eq!(b.cursor_line, 1);
    }

    #[test]
    pub fn move_cursor_left_skips_to_prev_line() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "argh foo bar");
        b.new_line(0);
        b.cursor_line = 1;
        b.goto_line_start();
        b.move_cursor_left();
        assert_eq!(b.cursor_position, 12);
        assert_eq!(b.cursor_line, 0);
    }

    #[test]
    pub fn can_merge_lines() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "fnord");
        inject_string(&mut b, "bar");
        assert_eq!(b.buffer.lines.len(), 3);
        assert_eq!(b.cursor_line, 2);
        b.cursor_line = 1;
        b.goto_line_start();
        b.remove_character(0);
        assert_eq!(b.buffer.lines.len(), 2);
        assert_eq!(b.cursor_line, 0);
        // should be right after the fnord!
        assert_eq!(b.cursor_position, 5);

        let ln = b.buffer.line_at(0).unwrap();
        assert_line_equals(ln, "fnordbar");
    }

    #[test]
    pub fn clear_selection_clears_selection() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "fnord");
        b.selection_start = Some((0, 0));
        b.selection_end = Some((0, 5));
        b.clear_selection();
        assert_eq!(b.selection_start, None);
        assert_eq!(b.selection_end, None);
    }

    #[test]
    pub fn add_character_to_selection() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "fnorda");
        b.cursor_line = 0;
        b.goto_line_end();
        b.selection_start = Some((0, 0));
        b.selection_end = Some((0, 5));
        b.extend_selection_to_cursor();
        assert_eq!(b.selection_start, Some((0, 0)));
        assert_eq!(b.selection_end, Some((0, 6)));
    }

    #[test]
    pub fn move_word_extend_selection_works_correctly() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "Foo To The Bar");
        b.cursor_line = 0;
        b.goto_line_start();
        b.selection_start = Some((0, 0));
        b.skip_word_forward();
        b.extend_selection_to_cursor();
        assert_eq!(b.selection_start, Some((0, 0)));
        assert_eq!(b.selection_end, Some((0, 3)));
        b.skip_word_forward();
        b.extend_selection_to_cursor();
        assert_eq!(b.selection_start, Some((0, 0)));
        assert_eq!(b.selection_end, Some((0, 6)));
    }

    #[test]
    pub fn move_word_extend_selection_backwards_shrinks_selection() {
        let mut b = BufferEntry::default();
        inject_string(&mut b, "Foo To The Bar");
        b.cursor_line = 0;
        b.goto_line_start();
        b.selection_start = Some((0, 0));
        b.skip_word_forward();
        b.extend_selection_to_cursor();
        assert_eq!(b.selection_start, Some((0, 0)));
        assert_eq!(b.selection_end, Some((0, 3)));
        b.skip_word_backward();
        b.extend_selection_to_cursor();
        assert_eq!(b.selection_start, Some((0, 0)));
        assert_eq!(b.selection_end, Some((0, 0)));
    }
}
