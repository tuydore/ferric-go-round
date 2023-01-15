use clap::Parser;

pub(crate) mod cli;

fn main() -> Result<(), anyhow::Error> {
    let cli = cli::raw::Cli::parse();
    let processed: cli::processed::ProcessedCli = cli.try_into()?;
    processed.save_carousel()?;
    processed.save_cover()?;
    Ok(())
}