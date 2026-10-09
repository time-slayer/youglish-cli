use clap::{Parser, ValueEnum};
use std::process;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Word or phrase to pronounce
    phrase: Vec<String>,

    /// Filter by a specific accent
    #[arg(short, long)]
    accent: Option<Accent>,

    /// Print generated URL instead of opening in browser
    #[arg(short, long)]
    print: bool,
    // TODO: implement shorthand flags for accents (--us, --uk and --aus)
    // #[arg(long)]
    //  us: bool,
    //
    // #[arg(long)]
    // uk: bool,
    //
    // #[arg(long)]
    // aus: bool,

    // TODO: add flag to split phrase into single-word queries
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

    if cli.print {
        println!("{url}");
    } else {
        open::that(url).unwrap();
    }
}
