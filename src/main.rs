use clap::{Parser, ValueEnum};
use std::process;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Word or phrase to pronounce
    phrase: Vec<String>,

    /// Specify accent to pronounce
    #[arg(short, long)]
    accent: Option<Accent>,

    // TODO: implement shorcut flags for accents (--us, --uk and --aus)
    // #[arg(long)]
    //  us: bool,
    //
    // #[arg(long)]
    // uk: bool,
    //
    // #[arg(long)]
    // aus: bool,
}

#[derive(Clone, ValueEnum)]
enum Accent {
    Us,
    Uk,
    Aus,
}

fn main() {
    let cli = Cli::parse();
    let phrase = cli.phrase.join("_");

    if phrase.trim().is_empty() {
        eprintln!("Please provide a non-empty phrase");
        eprintln!("Usage example: yg literally engage");
        process::exit(1);
    }

    let accent_path = match cli.accent {
        Some(Accent::Us) => "/us",
        Some(Accent::Uk) => "/uk",
        Some(Accent::Aus) => "/aus",
        None => "",
    };

    let url = format!(
        "https://youglish.com/pronounce/{}/english{}",
        phrase, accent_path
    );
    open::that(url).unwrap();
}
