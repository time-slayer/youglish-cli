use std::process;

use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Word or phrase to pronounce
    phrase: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let phrase = cli.phrase.join("_");

    if phrase.trim().is_empty() {
        eprintln!("Please provide a non-empty phrase");
        eprintln!("Usage example: yg literally engage");
        process::exit(1);
    }

    let url = format!("https://youglish.com/pronounce/{}/english", phrase);
    open::that(url).unwrap();
}
