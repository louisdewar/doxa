//! This is the binary that runs inside the VM

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use doxa_vm::executor::{
    spawn::{self, Python},
    VMExecutor,
};
use tokio::net::{TcpListener, UnixStream};
use tokio_vsock::VsockStream;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    subcommands: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Spawn {
        root: String,
        entrypoint: String,
    },
    #[clap(name = "vsock_listen")]
    VsockListen {
        #[clap(long)]
        cid: u32,
        #[clap(long)]
        port: u32,
    },
    #[clap(name = "tcp_listen")]
    TcpListen {
        #[clap(long, default_value = "\"0.0.0.0\"")]
        bind: String,
        #[clap(long, default_value = "1134")]
        port: u16,
    },

    #[clap(name = "unix_listen")]
    UnixListen {
        path: PathBuf,
    },
}

// Use single threaded runtime since this runs inside the VM which is locked to 1 CPU core anyway
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // This string is searched for by the recorder system to know when bootup is complete (for log truncating)
    println!("DOXA INIT started");

    let app = App::parse();
    match app.subcommands {
        SubCommands::Spawn { root, entrypoint } => {
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
        SubCommands::VsockListen { cid, port } => {
            let stream = VsockStream::connect(cid, port)
                .await
                .expect("Couldn't connect to stream");
            println!("VM executor connected");
            VMExecutor::start(stream).await.expect("VM executor failed");

            println!("VM Executor successfully completed");
        }
        SubCommands::TcpListen { bind, port } => {
            let listener = TcpListener::bind((bind, port))
                .await
                .expect("failed to start tcp listener");

            let (socket, socket_addr) = listener
                .accept()
                .await
                .expect("failed to accept connection");
            println!("Received TCP connect from {}", socket_addr);
            VMExecutor::start(socket).await.expect("VM executor failed");

            println!("VM Executor successfully completed");
        }
        SubCommands::UnixListen { path } => {
            let stream = UnixStream::connect(path)
                .await
                .expect("couldn't connect to socket");
            println!("VM executor connected");
            VMExecutor::start(stream).await.expect("VM executor failed");

            println!("VM Executor successfully completed");
        }
    }
}
