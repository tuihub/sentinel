mod config;
mod validator;

use std::{
    fs, io,
    io::{Read, Write},
    path::PathBuf,
    str,
};

use config::Config;
use glob::glob;

use crate::{
    err_msg,
    save_data::{config::BaseDirMode, validator::validate_config},
    Result,
};

///
pub struct CommonPath {
    document: PathBuf,
    profile: PathBuf,
    game: PathBuf,
}

const CONFIG_FILE_NAME: &str = "tuihub_savedata_config.json";

///
pub fn restore(common: CommonPath, file_path: PathBuf) -> Result<()> {
    let file = fs::File::open(file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut config_file = archive.by_name(CONFIG_FILE_NAME)?;
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str)?;
    let config: Config = validate_config(&config_str)?;
    drop(config_file);
    for entry in config.entries {
        let files: Vec<String> = archive
            .file_names()
            .filter(|f| f.to_string().starts_with(&entry.id.to_string()))
            .map(|f| f.to_owned())
            .collect();
        if files.is_empty() {
            return Err(err_msg!("invalid entry"));
        }
        let target_dir = common.combine(&entry.base_dir_mode, &entry.base_dir);
        let target_dir = fs::canonicalize(target_dir)?;
        if entry.clear_base_dir_before_restore && target_dir.is_dir() {
            fs::remove_dir_all(&target_dir)?;
        }
        for file in files {
            let f = archive.by_name(&file)?;
            let p = target_dir.join(f.mangled_name().strip_prefix(&entry.id.to_string())?);
            fs::create_dir_all(p.parent().unwrap_or(&target_dir))?;
            fs::write(p, f.extra_data())?;
        }
    }
    Ok(())
}

///
pub fn store(common: CommonPath, file_path: PathBuf, config_str: String) -> Result<()> {
    let config: Config = validate_config(&config_str)?;
    if let Some(file_dir) = file_path.parent() {
        if !file_dir.exists() {
            fs::create_dir_all(file_dir)?;
        }
    }
    let file = fs::File::create(file_path)?;
    let mut archive = zip::ZipWriter::new(file);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    archive.start_file(CONFIG_FILE_NAME, options)?;
    archive.write_all(config_str.as_ref())?;
    for entry in config.entries {
        let target_dir = common.combine(&entry.base_dir_mode, &entry.base_dir);
        if !target_dir.is_dir() {
            return Err(err_msg!("invalid config"));
        }
        let target_dir = fs::canonicalize(target_dir)?;
        for pattern in entry.file_pattern {
            if let Some(file_pattern) = target_dir.join(pattern).to_str() {
                let files = glob(file_pattern)?;
                for file in files {
                    let file = file?;
                    let mut f = fs::File::open(&file)?;
                    let file_name =
                        PathBuf::from(&entry.id.to_string()).join(file.strip_prefix(&target_dir)?);
                    archive.start_file(file_name.to_string_lossy(), options)?;
                    io::copy(&mut f, &mut archive)?;
                }
            }
        }
    }
    Ok(())
}

impl CommonPath {
    fn combine(&self, mode: &BaseDirMode, path: &PathBuf) -> PathBuf {
        match mode {
            BaseDirMode::Document => self.document.clone().join(path),
            BaseDirMode::Profile => self.profile.clone().join(path),
            BaseDirMode::Game => self.game.clone().join(path),
        }
    }
}
