//! This is the binary that runs inside the VM

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use doxa_vm::executor::{
    spawn::{self, Python},
    VMExecutor,
};

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    subcommands: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    Spawn { root: String, entrypoint: String },
}

// Use single threaded runtime since this runs inside the VM which is locked to 1 CPU core anyway
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = App::parse();
    match app.subcommands {
        Some(SubCommands::Spawn { root, entrypoint }) => {
            spawn::spawn::<Python>(
                &PathBuf::from(root),
                &Default::default(),
                &entrypoint,
                vec![],
            )
            .unwrap()
            .wait()
            .await
            .unwrap();
        }
        None => {
            VMExecutor::start(2, 1001)
                .await
                .expect("VM executor failed");

            println!("VM Executor successfully completed");
        }
    }
}
