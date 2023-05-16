use clap::Parser;
use granite2::model_checker::lola;
use granite2::{OutputFormat, PetriNet};
use log::info;

const ERR_SOURCE_FILE_NOT_FOUND: i32 = 1;
const ERR_OUTPUT_FOLDER_NOT_FOUND: i32 = 2;
const ERR_TRANSLATION: i32 = 3;
const ERR_OUTPUT_FILE_GENERATION: i32 = 4;

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Convert a Rust source code file into a Petri net \
    and export the resulting net in one of the supported formats.")]
struct CliArgs {
    /// The path to the Rust source code file to read.
    path: std::path::PathBuf,

    /// Filename for the resulting net.
    /// The output files contain this filename followed by an extension depending on the format.
    #[arg(long, default_value = "net")]
    filename: String,

    /// The path to a valid folder where the output files should be created.
    /// If not specified, the current working directory is used.
    #[arg(long, default_value = ".")]
    output_folder: std::path::PathBuf,

    /// The format for the output. Multiple formats can be specified.
    #[arg(long, value_enum)]
    format: Vec<OutputFormat>,

    /// If set, a reachability analysis to find deadlocks is performed.
    #[arg(long)]
    deadlock_analysis: bool,

    /// Verbosity flag.
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = CliArgs::parse();
    // Initialize an `env_logger` with the clap verbosity flag entered by the user.
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    // Double check that the file exists before starting the compiler
    // to generate an error message independent of the rustc output.
    info!(
        "Checking that the source code file at {} exists...",
        args.path.to_string_lossy()
    );
    if !args.path.exists() {
        eprintln!(
            "Source code file at {} does not exist",
            &args.path.to_string_lossy()
        );
        std::process::exit(ERR_SOURCE_FILE_NOT_FOUND);
    };

    // Double check that the output folder exists before starting the compiler
    // to generate an error message as soon as possible.
    info!(
        "Checking that the output folder at {} exists...",
        args.output_folder.to_string_lossy()
    );
    if !args.output_folder.exists() {
        eprintln!(
            "Output folder at {} does not exist",
            &args.output_folder.to_string_lossy()
        );
        std::process::exit(ERR_OUTPUT_FOLDER_NOT_FOUND);
    };

    info!("Starting the translation...");
    let petri_net = match granite2::run(args.path) {
        Ok(petri_net) => petri_net,
        Err(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(ERR_TRANSLATION);
        }
    };

    if let Err(err_str) = create_output_files(
        &petri_net,
        &args.filename,
        &args.output_folder,
        &args.format,
    ) {
        eprintln!("{err_str}");
        std::process::exit(ERR_OUTPUT_FILE_GENERATION);
    }

    if args.deadlock_analysis {
        let mut filepath = args.output_folder;
        filepath.push(&args.filename);
        filepath.set_extension(OutputFormat::Lola.to_string());

        if lola::check_deadlock(&filepath) {
            println!("Result: Deadlock can be reached according to the model checker `LoLA`");
        } else {
            println!("Result: The program is deadlock-free according to the model checker `LoLA`");
        }
    }
}

fn create_output_files(
    petri_net: &PetriNet,
    filename: &str,
    output_folder: &std::path::Path,
    format: &Vec<OutputFormat>,
) -> Result<(), std::io::Error> {
    for format in format {
        format.create_output_file(petri_net, filename, output_folder)?;
    }
    Ok(())
}
