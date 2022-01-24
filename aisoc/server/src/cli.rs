use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct App {
    #[clap(long, env)]
    pub dataset_x_image: PathBuf,
    #[clap(long, env)]
    pub dataset_y_folder: PathBuf,
    #[clap(long, env)]
    /// The path to the python binary that is used for running the scorer script
    pub scorer_python_bin: PathBuf,
}
