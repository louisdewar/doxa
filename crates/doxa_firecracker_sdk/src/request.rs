use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct BootSource {
    pub kernel_image_path: String,
    pub boot_args: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MountDrive {
    pub drive_id: String,
    pub path_on_host: String,
    pub is_root_device: bool,
    pub is_read_only: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateVsock {
    pub vsock_id: String,
    pub guest_cid: u32,
    pub uds_path: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Action {
    pub action_type: String,
}
