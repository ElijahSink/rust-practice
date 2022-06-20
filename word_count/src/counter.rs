use std::fs::{self, File};
use std::io::{BufRead, BufReader, ErrorKind, Result};

static FIELD_ORDER: [&str; 4] = ["lines", "words", "chars", "bytes"];

#[derive(Debug, Clone)]
pub struct CounterSettings {
    count_bytes: bool,
    count_chars: bool,
    count_words: bool,
    count_lines: bool,
    counted_fields: Box<Vec<String>>,
}

impl CounterSettings {
    pub fn new(count_bytes: bool, count_chars: bool, count_words: bool, count_lines: bool) -> Self {
        let fields = [count_lines, count_words, count_chars, count_bytes];
        let counted_fields: Vec<String> = fields
            .iter()
            .zip(FIELD_ORDER.iter())
            .filter(|(counted, _)| **counted)
            .map(|(_, name)| String::from(*name))
            .collect();

        let counted_fields = Box::new(counted_fields);

        CounterSettings {
            count_bytes,
            count_chars,
            count_words,
            count_lines,
            counted_fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    settings: CounterSettings,
    lines: u64,
    words: u64,
    chars: u64,
    bytes: u64,
    name: String,
    err: Option<String>,
}

impl Counter {
    pub fn new(name: String, settings: CounterSettings) -> Self {
        Counter {
            settings,
            bytes: 0,
            chars: 0,
            words: 0,
            lines: 0,
            name,
            err: None,
        }
    }

    pub fn index(&self, key: &str) -> u64 {
        match key {
            "bytes" => self.bytes,
            "chars" => self.chars,
            "words" => self.words,
            "lines" => self.lines,
            _ => panic!("Unexpected key: {}", key),
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
                    if self.settings.count_bytes {
                        self.bytes += bytes_read as u64;
                    }
                    if self.settings.count_chars {
                        self.chars += buff.chars().count() as u64;
                    }
                    if self.settings.count_words {
                        self.words += buff.split_ascii_whitespace().count() as u64;
                    }
                    if self.settings.count_lines {
                        self.lines += 1;
                    }
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

#[derive(Debug)]
pub struct FileCounter {
    pub counts: Counter,
    pub filepath: String,
}

impl FileCounter {
    pub fn new(filepath: String, settings: CounterSettings) -> Self {
        FileCounter {
            counts: Counter::new(filepath.clone(), settings),
            filepath,
        }
    }

    pub fn count(&mut self) -> Result<()> {
        let metadata = fs::metadata(self.filepath.as_str())?;
        let filetype = metadata.file_type();
        if filetype.is_dir() {
            self.counts.err = Some(format!("wc: {}: is a directory", self.filepath))
        } else {
            let f = File::open(self.filepath.as_str())?;
            let reader = BufReader::new(f);
            self.counts.count_stream(reader);
        }
        Ok(())
    }
}

pub fn display_counters(counters: &mut Vec<Counter>, settings: CounterSettings) {
    let mut totals = Counter::new(String::from("total"), settings.clone());
    let mut all_counts_for_width: Vec<u64> = Vec::new();

    for counter in counters.iter() {
        if counter.err.is_some() {
            continue;
        }
        totals.add(&counter);
        for counted_field in settings.counted_fields.iter() {
            all_counts_for_width.push(counter.index(counted_field));
        }
    }

    if counters.len() > 1 {
        counters.push(totals.clone());
        all_counts_for_width.extend(
            settings
                .counted_fields
                .iter()
                .map(|name| totals.index(name)),
        );
    }

    let width = all_counts_for_width
        .iter()
        .map(|s| s.to_string().len())
        .max()
        .unwrap_or(1)
        .clone();

    for counter in counters.iter() {
        if counter.err.is_some() {
            println!("{}", counter.err.as_ref().unwrap())
        }
        for counted_field in settings.counted_fields.iter() {
            print!("{:width$} ", counter.index(counted_field), width = width)
        }
        print!("{}\n", counter.name);
    }
}
