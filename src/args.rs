use {
    crate::rect::Rect,
    clap::{Parser, ValueEnum},
    std::path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// The rect which the output image must fit.
    /// Either both dimensions (eg "500x250") or
    /// just one for a square (eg "750")
    #[arg(short, long)]
    pub fit: Rect,

    #[arg(short, long, value_enum, default_value="normal")]
    pub verb: Verbosity,

    #[arg(long, default_value_t=85)]
    pub jpeg_quality: u8,

    pub source: Vec<PathBuf>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Verbosity {
    Silent,
    Normal,
    Verbose,
    VeryVerbose,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Normal
    }
}



