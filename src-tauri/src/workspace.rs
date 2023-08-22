use std::path::{Path, PathBuf};

use anyhow::anyhow;
use home_dir::HomeDirExt;

pub fn workspace_dir() -> anyhow::Result<PathBuf> {
    Path::new("~/.db-diff").expand_home().map_err(|e| anyhow!(e))
}

pub fn workspace_path(s: &str) -> anyhow::Result<PathBuf> {
    workspace_dir().map(|dir| dir.join(s))
}
