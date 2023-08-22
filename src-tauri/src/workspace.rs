use std::path::PathBuf;

use anyhow::anyhow;
use directories::UserDirs;

pub fn workspace_dir() -> anyhow::Result<PathBuf> {
    match UserDirs::new() {
        Some(user_dirs) => Ok(user_dirs.home_dir().join(".db-diff")),
        None => Err(anyhow!("home dir expansion failed.")),
    }
}

pub fn workspace_path(s: &str) -> anyhow::Result<PathBuf> {
    workspace_dir().map(|dir| dir.join(s))
}
