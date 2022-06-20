mod counter;

use counter::{display_counters, Counter, CounterSettings, FileCounter};

use clap::{AppSettings, ArgAction, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

struct Cli {
    #[clap(value_parser, multiple = true, help = "Input files")]
    file: Vec<String>,

    #[clap(short = 'c', long = "bytes", action=ArgAction::Set, help = "Print the byte counts")]
    count_bytes: Option<bool>,

    #[clap(
        short = 'm',
        long = "chars",
        action=ArgAction::Set,
        help = "Print the character counts"
    )]
    count_chars: Option<bool>,

    #[clap(short = 'w', long = "words", action=ArgAction::Set, help = "Print the word counts")]
    count_words: Option<bool>,

    #[clap(short = 'l', long = "lines", action=ArgAction::Set, help = "Print the newline counts")]
    count_lines: Option<bool>,
}

fn parse_settings(cli: &Cli) -> CounterSettings {
    let any_specified = [
        cli.count_bytes,
        cli.count_chars,
        cli.count_words,
        cli.count_lines,
    ]
    .iter()
    .any(|opt| opt.is_some());

    // if any are specified, everything else becomes false
    // if none are specified, everything is the default value

    if any_specified {
        CounterSettings::new(
            cli.count_bytes.unwrap_or(false),
            cli.count_chars.unwrap_or(false),
            cli.count_words.unwrap_or(false),
            cli.count_lines.unwrap_or(false),
        )
    } else {
        CounterSettings::new(true, false, true, true)
    }
}

fn main() {
    let cli = Cli::parse();

    let settings = parse_settings(&cli);

    let mut counters: Vec<Counter> = Vec::new();

    for file in cli.file {
        let mut fc = FileCounter::new(file.to_string(), settings.clone());
        fc.count().unwrap();
        counters.push(fc.counts);
    }
    display_counters(&mut counters, settings)
}
