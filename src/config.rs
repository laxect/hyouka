use dirs::config_dir;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    print::{section, Check},
    verbose,
};

static TARGET_DIR: OnceCell<PathBuf> = OnceCell::new();
static WORKING_DIR: OnceCell<PathBuf> = OnceCell::new();

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
        .join("config.toml");
    verbose!("load config from default path {}", config_path.to_string_lossy());
    let mut config = std::fs::File::open(config_path).verbose()?;
    let mut content = Vec::new();
    verbose!("read file content");
    config.read_to_end(&mut content).verbose()?;
    verbose!("parse config");
    let Config {
        target_dir,
        working_dir,
    } = toml::from_slice(&content).verbose()?;
    TARGET_DIR.set(target_dir).expect("Double init!");
    WORKING_DIR.set(working_dir).expect("Double init!");
    Ok(())
}

pub(crate) fn target_dir() -> &'static Path {
    TARGET_DIR.get().expect("Use before init")
}

pub(crate) fn working_dir() -> &'static Path {
    WORKING_DIR.get().expect("Use before init")
}
