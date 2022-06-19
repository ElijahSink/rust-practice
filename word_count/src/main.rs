mod counter;
use counter::{Counter, FileCounter};

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser, multiple = true, help = "Input files")]
    file: Vec<String>,
}
fn main() {
    let cli = Cli::parse();

    let mut totals = Counter::new();

    for file in cli.file {
        let mut fc = FileCounter::new(file.to_string());
        fc.count().unwrap();
        totals.add(&fc.counts);
        println!("{}", fc);
    }
    println!("{} total", totals);
}
