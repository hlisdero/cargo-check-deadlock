//! Submodule for the output formats supported by the tool.
use clap::ValueEnum;
use log::info;

use cargo_check_deadlock::PetriNet;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    /// Petri Net Markup Language - <https://www.pnml.org/>
    Pnml,
    /// LoLA - A Low Level Petri Net Analyzer - <https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/>
    Lola,
    /// DOT (graph description language) - <https://graphviz.org/>
    Dot,
}

impl OutputFormat {
    /// Converts a Petri net to an output file named `filename` in the given output folder.
    ///
    /// # Errors
    ///
    /// If the file cannot be created, then the function returns an error.
    /// If the Petri net cannot be written to the file, then the function returns an error.
    pub fn create_output_file(
        self,
        petri_net: &PetriNet,
        filename: &str,
        output_folder: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        let mut filepath = output_folder.to_path_buf();
        filepath.push(filename);
        filepath.set_extension(self.to_string());

        info!("Creating output file {}...", filepath.to_string_lossy());
        let mut file = std::fs::File::create(filepath)?;
        match self {
            Self::Dot => petri_net.to_dot(&mut file),
            Self::Lola => petri_net.to_lola(&mut file),
            Self::Pnml => petri_net.to_pnml(&mut file),
        }
    }
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
