use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Result};

#[derive(Debug)]
pub struct Counter {
    bytes: u64,
    chars: u64,
    words: u64,
    lines: u64,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            bytes: 0,
            chars: 0,
            words: 0,
            lines: 0,
        }
    }

    fn count_stream(&mut self, mut stream: BufReader<File>) {
        loop {
            let mut buff = String::new();
            match stream.read_line(&mut buff) {
                Ok(0) => {
                    return;
                }
                Ok(bytes_read) => {
                    self.bytes += bytes_read as u64;
                    self.chars += buff.chars().count() as u64;
                    self.words += buff.split_ascii_whitespace().count() as u64;
                    self.lines += 1;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                _ => {}
            }
        }
    }

    pub fn add(&mut self, other: &Counter) {
        self.bytes += other.bytes;
        self.chars += other.chars;
        self.words += other.words;
        self.lines += other.lines;
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = [self.lines, self.words, self.chars, self.bytes]
            .map(|s| s.to_string().len())
            .iter()
            .max()
            .unwrap()
            .clone();
        write!(
            f,
            "{:width$} {:width$} {:width$} {:width$}",
            self.lines,
            self.words,
            self.chars,
            self.bytes,
            width = width
        )
    }
}

#[derive(Debug)]
pub struct FileCounter {
    pub counts: Counter,
    pub filepath: String,
}

impl FileCounter {
    pub fn new(filepath: String) -> Self {
        FileCounter {
            counts: Counter::new(),
            filepath,
        }
    }

    pub fn count(&mut self) -> Result<()> {
        let f = File::open(self.filepath.as_str())?;
        let reader = BufReader::new(f);
        self.counts.count_stream(reader);
        Ok(())
    }
}

impl std::fmt::Display for FileCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.counts, self.filepath)
    }
}
