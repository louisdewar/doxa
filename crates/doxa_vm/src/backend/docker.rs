use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

pub use bollard::auth::DockerCredentials;
use bollard::{
    container::RemoveContainerOptions,
    image::CreateImageOptions,
    models::{DeviceRequest, HostConfig},
};

use futures_util::TryStreamExt;
use tokio::{
    io::empty,
    net::{UnixListener, UnixStream},
};
use tracing::info;

use crate::{
    error::{ManagerError, VMShutdownError},
    mount::Mount,
    recorder::VMRecorder,
    stream::Stream,
};

use super::VMBackend;

pub use bollard::Docker;

#[derive(Clone)]
pub struct DockerBackendSettings {
    pub image: String,
    pub docker: bollard::Docker,
    pub credentials: Option<DockerCredentials>,
    pub runtime: Option<String>,
    pub memory_limit_bytes: Option<i64>,
    //cpus: u64,
}

pub struct DockerBackend {
    docker: bollard::Docker,
    container_id: String,
    listener: UnixListener,
}

#[async_trait::async_trait]
impl VMBackend for DockerBackend {
    type BackendSettings = DockerBackendSettings;

    type Socket = UnixStream;

    const SUPPORTS_MOUNTING_IMAGES: bool = false;

    async fn spawn(
        tempdir: &Path,
        backend_settings: Self::BackendSettings,
        _ram_mib: u64,
        mounts: Vec<Mount>,
        _swap_path: PathBuf,
    ) -> Result<Self, ManagerError> {
        let docker = backend_settings.docker;
        let image = backend_settings.image;

        info!("test1");
        docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: image.clone(),
                    ..Default::default()
                }),
                None,
                backend_settings.credentials,
            )
            .try_collect::<Vec<_>>()
            .await
            .map_err(ManagerError::Bollard)?;

        let sock_folder = tempdir.join("sock");
        tokio::fs::create_dir_all(&sock_folder).await?;
        let doxa_sock_host = sock_folder.join("doxa.sock");
        let listener = UnixListener::bind(&doxa_sock_host)?;

        let mut binds: Vec<_> = mounts
            .into_iter()
            .map(|mount| {
                format!(
                    "{}:{}:{}",
                    mount.path_on_host.to_string_lossy(),
                    mount.path_on_guest,
                    if mount.read_only { "ro" } else { "rw" }
                )
            })
            .collect();
        binds.push(dbg!(format!(
            "{}:/doxa_sock:rw",
            sock_folder.to_string_lossy()
        )));

        let mut volumes = HashMap::new();
        volumes.insert(
            sock_folder.to_string_lossy().to_string(),
            Default::default(),
        );

        let container_config = bollard::container::Config {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            network_disabled: Some(true),
            tty: Some(true),
            image: Some(image),
            volumes: Some(volumes),
            host_config: Some(HostConfig {
                // ignore ram_mib
                memory: backend_settings.memory_limit_bytes,
                // temporarily ignore cpu limits
                // cpu_period: Some(100000),
                // cpu_quota: Some(100000 * backend_settings.cpus as i64),
                runtime: backend_settings.runtime,
                binds: Some(binds),
                publish_all_ports: Some(true),
                device_requests: Some(vec![DeviceRequest {
                    driver: Some("".to_string()),
                    count: Some(-1),
                    device_ids: None,
                    capabilities: Some(vec![vec!["gpu".to_string()]]),
                    options: None,
                }]),
                ..Default::default()
            }),
            entrypoint: Some(vec![
                "/sbin/vm_executor".to_string(),
                "unix_listen".to_string(),
                "/doxa_sock/doxa.sock".into(),
                // "tcp_listen".to_string(),
                // "--bind".into(),
                // "127.0.0.1".into(),
                // "--port".into(),
                // "1134".into(),
            ]),
            //volumes: todo!(),
            //network_disabled: todo!(),
            ..Default::default()
        };

        info!("test3");

        let container = docker
            .create_container::<String, String>(None, container_config)
            .await
            .map_err(ManagerError::Bollard)?;
        info!("test4");

        let container_id = container.id;

        docker
            .start_container::<String>(&container_id, None)
            .await
            .map_err(ManagerError::Bollard)?;

        info!("test5");
        info!(%container_id, "started container");
        Ok(DockerBackend {
            docker,
            container_id,
            listener,
        })
    }

    fn take_recorder(&mut self, max_len: usize) -> VMRecorder {
        // Temporarily have empty logs
        // In future:
        // docker
        //     .attach_container::<&str>(
        //         &container_id,
        //         Some(AttachContainerOptions {
        //             stdout: Some(true),
        //             stderr: Some(true),
        //             stream: Some(true),
        //             // Returns past logs
        //             logs: Some(true),
        //             ..Default::default()
        //         }),
        //     )
        //     .await
        //     .map_err(ManagerError::Bollard)?;
        VMRecorder::start(empty(), empty(), max_len)
    }

    /// Connects to the VM manager inside the VM.
    async fn connect(&mut self) -> Result<Stream<Self::Socket>, ManagerError> {
        // TODO: timeout
        let (stream, _) = self.listener.accept().await?;
        let mut stream = Stream::from_socket(stream);
        stream.send_full_message(b"NOMOUNTREQUEST_").await?;

        Ok(stream)
        // let networks = self
        //     .docker
        //     .inspect_container(&self.container_id, None)
        //     .await
        //     .map_err(ManagerError::Bollard)?
        //     .network_settings
        //     .ok_or(ManagerError::DockerMissingResponse)?
        //     .networks
        //     .ok_or(ManagerError::DockerMissingResponse)?;

        // let bridge = networks
        //     .get("bridge")
        //     .ok_or(ManagerError::MissingBridgeNetwork)?;

        // let ip = bridge
        //     .ip_address
        //     .clone()
        //     .ok_or(ManagerError::DockerMissingResponse)?;

        // dbg!(&ip);

        // let mut i = 0;
        // loop {
        //     dbg!(i);
        //     let socket = match TcpStream::connect((ip.as_str(), 1134)).await {
        //         Ok(socket) => socket,
        //         Err(e) => {
        //             if i == 5 {
        //                 break Err(e.into());
        //             }

        //             i += 1;
        //             tokio::time::sleep(Duration::from_secs(1)).await;
        //             continue;
        //         }
        //     };
        //     break Ok(Stream::from_socket(socket));
        // }
    }

    async fn shutdown(self) -> Result<(), VMShutdownError> {
        self.docker
            .remove_container(
                &self.container_id,
                Some(RemoveContainerOptions {
                    v: true,
                    force: true,
                    link: false,
                }),
            )
            .await
            .map_err(VMShutdownError::Bollard)?;

        Ok(())
    }
}
