/// Possible outcomes of running the `cargo check-deadlock` command.
pub enum CargoResult {
    /// A successful translation containing the result of the deadlock analysis
    DeadlockAnalysis(String),
    /// A successful translation without deadlock analysis
    SimpleTranslation,
    /// The source file was not found
    SourceFileNotFound(String),
    /// The output folder was not found
    OutputFolderNotFound(String),
    /// The translation failed
    TranslationError(String),
    /// Failure when writing the output files
    OutputGenerationError(String),
}
