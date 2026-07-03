use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;

/// Appends a timestamped line to logs/connl.log, creating the folder/file as needed.
// ponytail: single flat log file, no rotation — add rotation if it grows unwieldy.
pub fn log(msg: &str) {
    let _ = fs::create_dir_all("logs");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("logs/connl.log") {
        let _ = writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
    }
}
