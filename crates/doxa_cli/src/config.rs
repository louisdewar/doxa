use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

pub fn default_config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Cannot determine the system's preferred config directory for this user")
        .join("doxa_cli")
}

fn profiles_file(config_dir: &Path) -> PathBuf {
    config_dir.join("profiles")
}

pub async fn ensure_config_dir_exists(config_dir: &Path) -> io::Result<()> {
    tokio::fs::create_dir_all(config_dir).await
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub auth_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProfileConfig {
    pub default: Option<String>,
    pub profiles: Vec<UserProfile>,
}

impl ProfileConfig {
    pub fn user_profile(&self, user: &str) -> Option<&UserProfile> {
        for profile in &self.profiles {
            if profile.name == user {
                return Some(profile);
            }
        }

        None
    }

    pub fn default_profile(&self) -> Option<&UserProfile> {
        self.default
            .as_ref()
            .and_then(|default| self.user_profile(default))
    }

    pub fn upsert_profile(&mut self, user: String, auth_token: String) {
        for profile in &mut self.profiles {
            if profile.name == user {
                profile.auth_token = auth_token;

                return;
            }
        }

        self.profiles.push(UserProfile {
            name: user,
            auth_token,
        });
    }
}

pub async fn load_or_default_profile(config_dir: &Path) -> io::Result<ProfileConfig> {
    let profiles = match tokio::fs::read_to_string(profiles_file(config_dir)).await {
        Ok(v) => v,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(ProfileConfig::default()),
        Err(e) => return Err(e),
    };

    let profiles: ProfileConfig =
        serde_json::from_str(&profiles).expect("improperly formatted profiles file");

    Ok(profiles)
}

pub async fn save_profile(config_dir: &Path, profiles: ProfileConfig) -> io::Result<()> {
    ensure_config_dir_exists(config_dir).await?;
    let mut f = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(profiles_file(config_dir))
        .await?;
    f.write_all(&serde_json::to_vec(&profiles).unwrap()).await
}
