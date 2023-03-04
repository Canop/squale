pub mod args;
pub mod output_format;
pub mod rect;
pub mod scaler;

use {
    anyhow::Result,
    clap::Parser,
};

fn main() -> Result<()> {
    let args = args::Args::parse();
    let scaler = scaler::Scaler::new(&args)?;
    scaler.handle(&args.source)?;
    Ok(())
}
