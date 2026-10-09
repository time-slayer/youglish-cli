mod cli;

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();
    let phrase = cli.phrase.join("_");

    if phrase.trim().is_empty() {
        eprintln!("Please provide a non-empty phrase");
        eprintln!("Usage example: yg literally engage");
        process::exit(1);
    }

    let accent = match cli.accent {
        Some(a) => a.as_path(),
        None => "",
    };

    let url = format!("https://youglish.com/pronounce/{phrase}/english{accent}");

    if cli.print {
        println!("{url}");
    } else {
        open::that(url).unwrap();
    }
}
