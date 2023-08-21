use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::anyhow;
use chrono::Local;
use home_dir::HomeDirExt;

pub fn info<S: Into<String>>(s: S) {
    let _ = write("INFO", s.into());
}

pub fn error<S: Into<String>>(s: S) {
    let _ = write("ERROR", s.into());
}

fn write(label: &str, s: String) -> anyhow::Result<()> {
    let path = Path::new("~/.db-diff/db-diff.log").expand_home().map_err(|e| anyhow!(e))?;
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(format!("{} {} {}\n", Local::now().format("%Y/%m/%d %H:%M:%S"), label, s).as_bytes()).map_err(|e| anyhow!(e))?;
    file.flush().map_err(|e| anyhow!(e))
}
