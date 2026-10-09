use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Word or phrase to pronounce
    pub phrase: Vec<String>,

    /// Filter by a specific accent
    #[arg(short, long)]
    pub accent: Option<Accent>,

    /// Print generated URL instead of opening in browser
    #[arg(short, long)]
    pub print: bool,
    //
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
pub enum Accent {
    Us,
    Uk,
    Aus,
}

impl Accent {
    pub fn as_path(self) -> &'static str {
        match self {
            Self::Us => "/us",
            Self::Uk => "/uk",
            Self::Aus => "/aus",
        }
    }
}
