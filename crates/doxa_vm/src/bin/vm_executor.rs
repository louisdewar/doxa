//! This is the binary that runs inside the VM

use doxa_vm::executor::VMExecutor;

// Use single threaded runtime since this runs inside the VM which is locked to 1 CPU core anyway
#[tokio::main(flavor = "current_thread")]
async fn main() {
    VMExecutor::start(2, 1001)
        .await
        .expect("VM executor failed");

    println!("VM Executor successfully completed");
}
