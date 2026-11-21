use clap::{Parser, ValueEnum};
use std::env;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Word or phrase to pronounce
    #[arg(required = true)]
    pub phrase: Vec<String>,

    /// Filter by a specific accent
    #[arg(short, long, group = "accent_filter")]
    accent: Option<Accent>,

    /// Shorthand for --accent us
    #[arg(long, group = "accent_filter")]
    us: bool,

    /// Shorthand for --accent uk
    #[arg(long, group = "accent_filter")]
    uk: bool,

    /// Shorthand for --accent aus
    #[arg(long, group = "accent_filter")]
    aus: bool,

    /// Print generated URL instead of opening in browser
    #[arg(short, long)]
    pub print: bool,

    /// Split phrase into single-word queries
    #[arg(short, long)]
    pub split: bool,
}

impl Cli {
    pub fn selected_accent(&self) -> Option<Accent> {
        match self {
            Self { us: true, .. } => Some(Accent::Us),
            Self { uk: true, .. } => Some(Accent::Uk),
            Self { aus: true, .. } => Some(Accent::Aus),
            _ => self.accent.or_else(Accent::from_env),
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
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

    fn from_env() -> Option<Self> {
        let accent = env::var("YG_ACCENT").ok()?;
        Self::from_str(accent.trim(), true).ok()
    }
}
