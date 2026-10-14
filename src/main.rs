mod cli;

use clap::Parser;
use cli::{Accent, Cli};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let phrase = cli.phrase.join("_");

    if phrase.trim().is_empty() {
        eprintln!("Please provide a non-empty phrase");
        eprintln!("Usage example: yg literally engage");
        process::exit(1);
    }

    let accent = cli.selected_accent().map(Accent::as_path).unwrap_or("");

    let url = format!("https://youglish.com/pronounce/{phrase}/english{accent}");

    if cli.print {
        println!("{url}");
    } else {
        open::that(url)?;
    }

    Ok(())
}
