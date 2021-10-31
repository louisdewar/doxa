use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadParams {
    #[serde(default)]
    pub active: bool,
}
