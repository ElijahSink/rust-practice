mod counter;

use counter::{display_counters, Counter, CounterSettings, FileCounter};

use clap::{AppSettings, ArgAction, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

struct Cli {
    #[clap(value_parser, multiple = true, help = "Input files")]
    file: Vec<String>,

    #[clap(short = 'c', long = "bytes", action=ArgAction::SetTrue, help = "Print the byte counts")]
    count_bytes: bool,

    #[clap(
        short = 'm',
        long = "chars",
        action=ArgAction::SetTrue,
        help = "Print the character counts"
    )]
    count_chars: bool,

    #[clap(short = 'w', long = "words", action=ArgAction::SetTrue, help = "Print the word counts")]
    count_words: bool,

    #[clap(short = 'l', long = "lines", action=ArgAction::SetTrue, help = "Print the newline counts")]
    count_lines: bool,
}

fn parse_settings(cli: &Cli) -> CounterSettings {
    let none_specified = [
        cli.count_bytes,
        cli.count_chars,
        cli.count_words,
        cli.count_lines,
    ]
    .iter()
    .all(|opt| !opt);

    // if any are specified, everything else becomes false
    // if none are specified, everything is the default value

    if none_specified {
        CounterSettings::new(true, false, true, true)
    } else {
        CounterSettings::new(
            cli.count_bytes,
            cli.count_chars,
            cli.count_words,
            cli.count_lines,
        )
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
