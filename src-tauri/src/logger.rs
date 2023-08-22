use std::fs::OpenOptions;
use std::io::Write;

use crate::workspace::workspace_path;
use anyhow::anyhow;
use chrono::Local;

pub fn info<S: Into<String>>(s: S) {
    let _ = write("INFO", s.into());
}

pub fn error<S: Into<String>>(s: S) {
    let _ = write("ERROR", s.into());
}

fn write(label: &str, s: String) -> anyhow::Result<()> {
    let path = workspace_path("db-diff.log")?;
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(format!("{} {} {}\n", Local::now().format("%Y/%m/%d %H:%M:%S"), label, s).as_bytes()).map_err(|e| anyhow!(e))?;
    file.flush().map_err(|e| anyhow!(e))
}
