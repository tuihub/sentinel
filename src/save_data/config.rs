use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub platform: Platform,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "windows")]
    Windows,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: u8,
    #[serde(rename = "baseDirMode")]
    pub base_dir_mode: BaseDirMode,
    #[serde(rename = "baseDir")]
    pub base_dir: PathBuf,
    #[serde(rename = "filePattern")]
    pub file_pattern: Vec<PathBuf>,
    #[serde(rename = "clearBaseDirBeforeRestore")]
    pub clear_base_dir_before_restore: bool,
}

#[derive(Serialize, Deserialize)]
pub enum BaseDirMode {
    #[serde(rename = "game-root")]
    Game,
    #[serde(rename = "user-document")]
    Document,
    #[serde(rename = "user-profile")]
    Profile,
}
