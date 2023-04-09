mod config;

use crate::save_data::config::PathMode;
use crate::{err_msg, Result};
use config::Config;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::{fs, io, path};
use walkdir::WalkDir;

///
pub struct CommonPath {
    document: PathBuf,
    profile: PathBuf,
}

///
pub fn restore(common: CommonPath, game_path: PathBuf, file_path: PathBuf) -> Result<()> {
    let file = fs::File::open(file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let config_file = archive.by_name("tuihub_savedata_config.json")?;
    let config: Config = serde_json::from_reader(config_file)?;
    if config.entries.is_empty() {
        return Err(err_msg!("empty entry"));
    }
    for entry in config.entries {
        let files: Vec<String> = archive
            .file_names()
            .filter(|f| f.to_string().starts_with(&entry.id.to_string()))
            .map(|f| f.to_owned())
            .collect();
        if files.is_empty() {
            return Err(err_msg!("invalid entry"));
        }
        let target_dir = match entry.path_mode {
            PathMode::Absolute => entry.path.to_owned(),
            PathMode::Document => common.document.clone().join(&entry.path),
            PathMode::Profile => common.profile.clone().join(&entry.path),
            PathMode::Game => game_path.clone().join(&entry.path),
        };
        if entry
            .path
            .as_os_str()
            .as_bytes()
            .ends_with(path::MAIN_SEPARATOR_STR.as_bytes())
        {
            for file in files {
                let f = archive.by_name(&file)?;
                let p = target_dir.join(f.mangled_name().strip_prefix(&entry.id.to_string())?);
                fs::create_dir_all(p.parent().unwrap_or(&target_dir))?;
                fs::write(p, f.extra_data())?;
            }
        } else if let Some(name) = files.get(0) {
            let f = archive.by_name(name)?;
            fs::create_dir_all(target_dir.parent().unwrap_or(&target_dir))?;
            fs::write(target_dir, f.extra_data())?;
        } else {
            return Err(err_msg!("invalid entry"));
        }
    }
    Ok(())
}

///
pub fn store(
    common: CommonPath,
    game_path: PathBuf,
    config_str: String,
    file_path: PathBuf,
) -> Result<()> {
    if let Some(file_dir) = file_path.parent() {
        if !file_dir.exists() {
            fs::create_dir_all(file_dir)?;
        }
    }
    let config: Config = serde_json::from_str(&config_str)?;
    let file = fs::File::create(file_path)?;
    let mut archive = zip::ZipWriter::new(file);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    archive.start_file("tuihub_savedata_config.json", options)?;
    archive.write_all(config_str.as_ref())?;
    for entry in config.entries {
        let target_dir = match entry.path_mode {
            PathMode::Absolute => entry.path.to_owned(),
            PathMode::Document => common.document.clone().join(&entry.path),
            PathMode::Profile => common.profile.clone().join(&entry.path),
            PathMode::Game => game_path.clone().join(&entry.path),
        };
        if entry
            .path
            .as_os_str()
            .as_bytes()
            .ends_with(path::MAIN_SEPARATOR_STR.as_bytes())
        {
            if !target_dir.is_dir() {
                return Err(err_msg!("invalid config"));
            }
            let files: Vec<PathBuf> = WalkDir::new(&target_dir)
                .max_depth(10)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.into_path())
                .filter(|path| path.is_file())
                .collect();
            for file in files {
                let mut f = fs::File::open(file.clone())?;
                let file_name =
                    PathBuf::from(&entry.id.to_string()).join(file.strip_prefix(&target_dir)?);
                archive.start_file(file_name.to_string_lossy(), options)?;
                io::copy(&mut f, &mut archive)?;
            }
        } else {
            if !target_dir.is_file() {
                return Err(err_msg!("invalid config"));
            }
            let mut f = fs::File::open(&target_dir)?;
            let file_name =
                PathBuf::from(&entry.id.to_string()).join(entry.path.strip_prefix(".")?);
            archive.start_file(file_name.to_string_lossy(), options)?;
            io::copy(&mut f, &mut archive)?;
        }
    }
    Ok(())
}