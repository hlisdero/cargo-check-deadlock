use clap::Parser;
use log::info;

use crate::cargo_result::CargoResult;
use crate::output_format::OutputFormat;

use cargo_check_deadlock::model_checker::lola;
use cargo_check_deadlock::PetriNet;

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Debug, Parser)]
pub struct Args {
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

impl Args {
    pub fn exec(&self) -> CargoResult {
        // Initialize an `env_logger` with the clap verbosity flag entered by the user.
        env_logger::Builder::new()
            .filter_level(self.verbose.log_level_filter())
            .init();

        // Double check that the file exists before starting the compiler
        // to generate an error message independent of the rustc output.
        info!(
            "Checking that the source code file at {} exists...",
            self.path.to_string_lossy()
        );
        if !self.path.exists() {
            let err_str = format!(
                "Source code file at {} does not exist",
                &self.path.to_string_lossy()
            );
            return CargoResult::SourceFileNotFound(err_str);
        };

        // Double check that the output folder exists before starting the compiler
        // to generate an error message as soon as possible.
        info!(
            "Checking that the output folder at {} exists...",
            self.output_folder.to_string_lossy()
        );
        if !self.output_folder.exists() {
            let err_str = format!(
                "Output folder at {} does not exist",
                &self.output_folder.to_string_lossy()
            );
            return CargoResult::OutputFolderNotFound(err_str);
        };

        info!("Starting the translation...");
        let petri_net = match cargo_check_deadlock::run(self.path.clone()) {
            Ok(petri_net) => petri_net,
            Err(err_str) => {
                return CargoResult::TranslationError(err_str.to_string());
            }
        };

        let result = create_output_files(
            &petri_net,
            &self.filename,
            &self.output_folder,
            &self.format,
        );
        if let Err(err_str) = result {
            return CargoResult::OutputGenerationError(err_str.to_string());
        }

        if self.deadlock_analysis {
            let mut filepath = self.output_folder.clone();
            filepath.push(&self.filename);
            filepath.set_extension(OutputFormat::Lola.to_string());

            let message = if lola::check_deadlock(&filepath) {
                "Deadlock can be reached according to the model checker `LoLA`"
            } else {
                "The program is deadlock-free according to the model checker `LoLA`"
            };
            CargoResult::DeadlockAnalysis(message.to_string())
        } else {
            CargoResult::SimpleTranslation
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
