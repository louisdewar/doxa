use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct App {
    /// The path to the directory containing the climate hack datasets (multiple datasets in this
    /// directory)
    #[clap(long, env)]
    pub climatehack_datasets_dir: PathBuf,
    /// The name of the primary dataset that climate hack will use.
    #[clap(long, env)]
    pub climatehack_primary_dataset: String,
    #[clap(long, env)]
    /// The path to the python binary that is used for running the scorer script
    pub scorer_python_bin: PathBuf,
}
