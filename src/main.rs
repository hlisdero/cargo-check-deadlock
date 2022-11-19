use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use log::info;
use std::fs::File;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Petri Net Markup Language - <https://www.pnml.org/>
    Pnml,
    /// LoLA - A Low Level Petri Net Analyzer - A model checker by the Universität Rostock
    Lola,
    /// DOT (graph description language)
    Dot,
}

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
#[command(about = "Convert a Rust source code file into a Petri net \
    and export the resulting net in one of the supported formats.")]
struct CliArgs {
    /// The path to the Rust source code file to read
    path: std::path::PathBuf,

    /// The format for the output
    #[arg(short, long, value_enum)]
    output_format: Vec<OutputFormat>,

    /// If present or set to true, dump pretty printer MIR (Mid-level IR) into the given file
    #[arg(long, default_value_t = false)]
    mir_dump: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    info!("Parsing arguments");
    let args = CliArgs::parse();
    info!("Opening file");
    let _file = File::open(&args.path)
        .with_context(|| format!("Could not open file `{}`", args.path.display()))?;

    println!("CLI args: {:?}", args);

    Ok(())
}
