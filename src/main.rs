mod cli;

use clap::Parser;
use cli::{Accent, Cli};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Check if all words in phrase are empty strings
    if cli.phrase.join("").trim().is_empty() {
        eprintln!("Please provide a non-empty phrase");
        eprintln!("Usage example: yg literally engage");
        process::exit(1);
    }

    let queries: Vec<String> = if cli.split {
        cli.phrase.clone()
    } else {
        vec![cli.phrase.join("_")]
    };

    let accent = cli.selected_accent().map(Accent::as_path).unwrap_or("");

    for query in queries {
        let url = format!("https://youglish.com/pronounce/{query}/english{accent}");

        if cli.print {
            println!("{url}");
        } else {
            open::that(url)?;
        }
    }

    Ok(())
}
