use clap::{Parser, ValueEnum};
use log::info;
use netcrab::petri_net::PetriNet;

const ERR_SOURCE_FILE_NOT_FOUND: i32 = 1;
const ERR_TRANSLATION: i32 = 2;
const ERR_OUTPUT_FILE_GENERATION: i32 = 3;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Petri Net Markup Language - <https://www.pnml.org/>
    Pnml,
    /// LoLA - A Low Level Petri Net Analyzer - A model checker by the Universität Rostock
    Lola,
    /// DOT (graph description language)
    Dot,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dot => write!(f, "dot"),
            Self::Lola => write!(f, "lola"),
            Self::Pnml => write!(f, "pnml"),
        }
    }
}

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Convert a Rust source code file into a Petri net \
    and export the resulting net in one of the supported formats.")]
struct CliArgs {
    /// The path to the Rust source code file to read
    path: std::path::PathBuf,

    /// The format for the output
    #[arg(short, long, value_enum)]
    output_format: Vec<OutputFormat>,
}

fn main() {
    env_logger::init();

    info!("Parsing arguments");
    let args = CliArgs::parse();

    info!("Checking that the source code file exists");
    // Double check that the file exists before starting the compiler
    // to generate an error message independent of the rustc output.
    if !args.path.exists() {
        eprintln!(
            "Source code file at {} does not exist",
            &args.path.to_string_lossy()
        );
        std::process::exit(ERR_SOURCE_FILE_NOT_FOUND);
    };

    info!("Starting compiler");
    let petri_net = match granite2::run(args.path) {
        Ok(petri_net) => petri_net,
        Err(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(ERR_TRANSLATION);
        }
    };

    info!("Generating output files");
    if let Err(err_str) = create_output_files(&petri_net, &args.output_format) {
        eprintln!("{err_str}");
        std::process::exit(ERR_OUTPUT_FILE_GENERATION);
    }
}

fn create_output_files(
    petri_net: &PetriNet,
    output_format: &Vec<OutputFormat>,
) -> Result<(), std::io::Error> {
    for format in output_format {
        let filename = format!("net.{format}");
        let mut file = std::fs::File::create(filename)?;
        match format {
            OutputFormat::Dot => petri_net.to_dot(&mut file)?,
            OutputFormat::Lola => petri_net.to_lola(&mut file)?,
            OutputFormat::Pnml => petri_net.to_pnml(&mut file)?,
        }
    }
    Ok(())
}
