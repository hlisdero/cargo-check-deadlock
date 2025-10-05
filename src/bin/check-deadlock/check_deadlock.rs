use clap::Parser;
use log::info;

use crate::cargo_result::CargoResult;
use crate::output_format::OutputFormat;

use cargo_check_deadlock::model_checker::lola;

/// Convert a Rust source code file into a Petri net and export
/// the resulting net in one of the supported formats.
#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)] // We definitely need 3 bools here, the warning is excessive
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

    /// If set, outputs the Petri net in DOT format.
    #[arg(long)]
    dot: bool,

    /// If set, outputs the Petri net in PNML format.
    #[arg(long)]
    pnml: bool,

    /// If set, the reachability analysis to find deadlocks is skipped.
    #[arg(long)]
    skip_analysis: bool,

    /// If set, outputs the witness path, i.e., the series of transition firings that lead to the deadlock,
    /// to a file named `witness-path.txt`.
    #[arg(long)]
    witness_path: bool,

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
        }

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
        }

        info!("Starting the translation...");
        let petri_net = match cargo_check_deadlock::run(self.path.clone()) {
            Ok(petri_net) => petri_net,
            Err(err_str) => {
                return CargoResult::TranslationError(err_str.to_string());
            }
        };

        if self.dot {
            let format = OutputFormat::Dot;
            if let Err(err_str) =
                format.create_output_file(&petri_net, &self.filename, &self.output_folder)
            {
                return CargoResult::OutputGenerationError(err_str.to_string());
            }
        }

        if self.pnml {
            let format = OutputFormat::Pnml;
            if let Err(err_str) =
                format.create_output_file(&petri_net, &self.filename, &self.output_folder)
            {
                return CargoResult::OutputGenerationError(err_str.to_string());
            }
        }
        // Always generate the file in LoLA format for the deadlock analysis
        let format = OutputFormat::Lola;
        if let Err(err_str) =
            format.create_output_file(&petri_net, &self.filename, &self.output_folder)
        {
            return CargoResult::OutputGenerationError(err_str.to_string());
        }

        if self.skip_analysis {
            return CargoResult::SimpleTranslation;
        }

        let mut filepath = self.output_folder.clone();
        filepath.push(&self.filename);
        filepath.set_extension(OutputFormat::Lola.to_string());

        let witness_path = if self.witness_path {
            let mut path = self.output_folder.clone();
            path.push("witness-path.txt");
            Some(path)
        } else {
            None
        };

        let message = if lola::check_deadlock(&filepath, witness_path.as_ref()) {
            "Deadlock can be reached according to the model checker `LoLA`"
        } else {
            "The program is deadlock-free according to the model checker `LoLA`"
        };
        CargoResult::DeadlockAnalysis(message.to_string())
    }
}
