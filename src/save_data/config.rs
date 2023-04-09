use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    #[serde(rename = "pathMode")]
    pub path_mode: PathMode,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub enum PathMode {
    #[serde(rename = "absolute")]
    Absolute,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "profile")]
    Profile,
}
