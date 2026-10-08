use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Word or phrase to pronounce
    phrase: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let phrase = cli.phrase.join("_");
    let url = format!("https://youglish.com/pronounce/{}/english", phrase);
    open::that(url).unwrap();
}
