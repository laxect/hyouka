use dirs::config_dir;
use serde::Deserialize;
use std::{
    io::Read,
    lazy::SyncOnceCell,
    path::{Path, PathBuf},
};

use crate::print::*;

static TARGET_DIR: SyncOnceCell<PathBuf> = SyncOnceCell::new();
static WORKING_DIR: SyncOnceCell<PathBuf> = SyncOnceCell::new();

#[derive(Deserialize)]
pub struct Config {
    pub target_dir: PathBuf,
    pub working_dir: PathBuf,
}

pub fn load_config() -> anyhow::Result<()> {
    section("Config");
    let config_path = config_dir()
        .expect("No XDG_CONFIG_DIR setted")
        .join("hyouka")
        .join("config.yml");
    verbose(format!(
        "load config from default path {}",
        config_path.as_os_str().to_string_lossy()
    ));
    let mut config = std::fs::File::open(config_path).verbose()?;
    let mut content = Vec::new();
    verbose("read file content".into());
    config.read_to_end(&mut content).verbose()?;
    verbose("parse config".into());
    let Config {
        target_dir,
        working_dir,
    } = toml::from_slice(&content).verbose()?;
    TARGET_DIR.set(target_dir).expect("Double init!");
    WORKING_DIR.set(working_dir).expect("Double init!");
    Ok(())
}

pub(crate) fn target_dir() -> &'static Path {
    &TARGET_DIR.get().expect("Use before init")
}

pub(crate) fn working_dir() -> &'static Path {
    &WORKING_DIR.get().expect("Use before init")
}
