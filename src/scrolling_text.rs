use std::collections::VecDeque;
use std::io::{self, Write};

pub struct ScrollingText {
    lines: VecDeque<String>,
    max_lines: usize,
    is_max: bool,
}

impl ScrollingText {
    pub fn new(max_lines: usize) -> Self {
        ScrollingText {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
            is_max: false,
        }
    }

    pub fn append(&mut self, text: String) {
        // If we're at capacity, remove the first (oldest) line
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
            self.is_max = true;
        }

        // Add the new line at the end
        self.lines.push_back(text);

        // Print the current state
        self.print();
    }

    fn print(&mut self) {
        if self.is_max {
            // Move cursor up 4 lines to overwrite previous output
            print!("\x1B[{}A", self.max_lines);
            // Print all current lines
            for line in &self.lines {
                print!("\x1B[2K{}\n", line);
            }
        } else {
            // Before reaching max, only print the last line added
            if let Some(last_line) = self.lines.back() {
                print!("\x1B[2K{}\n", last_line);
            }
        }

        io::stdout().flush().unwrap();
    }
}
