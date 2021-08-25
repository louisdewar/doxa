//! This is a way of testing the execution system independantly of the MQ and the rest of the
//! system

use std::{path::PathBuf, time::Duration};

use doxa_vm::manager::Manager;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    std::env::set_current_dir(PathBuf::from("/home/louis/Documents/Testing/firecracker")).unwrap();

    assert!(PathBuf::from("./rootfs.img").exists());

    let mut manager = Manager::new(
        PathBuf::from("./rootfs.img"),
        PathBuf::from("./vmlinux.bin"),
        "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        PathBuf::from("./firecracker"),
    )
    .await
    .unwrap();

    let agent_path = PathBuf::from("agent.tar.gz");
    let agent = tokio::fs::OpenOptions::new()
        .read(true)
        .open(&agent_path)
        .await
        .unwrap();
    let metadata = agent.metadata().await.unwrap();
    let agent_size = metadata.len();

    manager
        .send_agent(
            agent_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            agent_size,
            agent,
        )
        .await
        .unwrap();

    println!("Done");
    tokio::time::sleep(Duration::from_secs(2)).await;
    drop(manager);
}
