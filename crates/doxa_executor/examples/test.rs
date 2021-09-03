use std::path::PathBuf;

use doxa_executor::{agent::Agent, Settings};
use doxa_storage::AgentRetrieval;
use doxa_vm::ExecutionConfig;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let agent_config = tokio::fs::read_to_string(PathBuf::from("./test_agent/doxa.toml"))
        .await
        .unwrap();

    let config = ExecutionConfig {
        entrypoint: "main.py".to_string(),
        language: doxa_vm::Language::Python,
        options: Default::default(),
    };
    dbg!(doxa_vm::serde_yaml::to_string(&config).unwrap());

    match doxa_vm::serde_yaml::from_str::<ExecutionConfig>(&agent_config) {
        Ok(config) => dbg!(config),
        Err(e) => {
            println!("{}", agent_config);
            panic!("doxa.toml file is invalid: {}", e);
        }
    };

    std::env::set_current_dir(PathBuf::from("/home/louis/Documents/Testing/firecracker")).unwrap();
    let retrieval = AgentRetrieval::new("http://localhost:3001/storage/download/".to_string());

    let agent = Agent::new(
        "helloworld",
        "462a6b558880af2482ff",
        &retrieval,
        Settings {
            firecracker_path: PathBuf::from("./firecracker"),
            kernel_img: PathBuf::from("./vmlinux.bin"),
            kernel_boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
            rootfs: PathBuf::from("./rootfs.img"),
        },
    )
    .await
    .unwrap();

    dbg!("SHUTDOWN");
    agent.shutdown().await.unwrap();
}
