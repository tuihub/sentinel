use std::{collections::HashMap, path::PathBuf};

use fancy_regex::{Match, Regex};
use log::{debug, warn};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct ScanResult {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
}

pub fn fixed_depth_mode(folder: PathBuf, depth: usize) -> Vec<ScanResult> {
    debug!("Start scan in fixed depth mode");
    WalkDir::new(folder)
        .min_depth(depth)
        .max_depth(depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|file| file.to_owned())
                .map(|file_name| (entry, file_name))
        })
        .map(|(entry, file_name)| {
            let mut path = entry.clone().into_path();
            path.pop();
            ScanResult {
                path,
                name: file_name,
                size: calculate_size(entry.into_path()),
            }
        })
        .collect()
}

fn calculate_size(folder: PathBuf) -> u64 {
    let size = WalkDir::new(folder.clone())
        .min_depth(0)
        .max_depth(20)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());
    debug!("Calculated {} size {}", folder.display(), size);
    size
}

pub fn single_file_mode(folder: PathBuf, max_depth: usize) -> Vec<ScanResult> {
    debug!("Start scan in single file mode");
    WalkDir::new(folder)
        .min_depth(0)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.metadata() {
            Ok(metadata) => Some((entry, metadata)),
            Err(_) => None,
        })
        .filter(|(_, metadata)| metadata.is_file())
        .filter_map(|(entry, metadata)| {
            entry
                .file_name()
                .to_str()
                .map(|file| file.to_owned())
                .map(|file_name| (entry, metadata, file_name))
        })
        .map(|(entry, metadata, file_name)| {
            let mut path = entry.into_path();
            path.pop();
            ScanResult {
                path,
                name: file_name,
                size: metadata.len(),
            }
        })
        .collect()
}

pub fn files_folder_mode(folder: PathBuf, max_depth: usize) -> Vec<ScanResult> {
    debug!("Start scan in files folder mode");
    let regex = Regex::new(r"[^/]+(?!.*/)").unwrap();
    WalkDir::new(folder)
        .min_depth(0)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.metadata() {
            Ok(metadata) => Some((entry, metadata)),
            Err(_) => None,
        })
        .filter(|(_, metadata)| metadata.is_file())
        .fold(
            HashMap::new(),
            |mut m: HashMap<PathBuf, ScanResult>, (entry, metadata)| {
                let mut path = entry.into_path();
                path.pop();
                let mut size = metadata.len();
                if let Some(v) = m.get(&path) {
                    size += v.size
                }
                m.insert(
                    path.to_owned(),
                    ScanResult {
                        path,
                        name: "".to_owned(),
                        size,
                    },
                );
                m
            },
        )
        .into_iter()
        .map(|(path, result)| {
            let name = regex
                .captures(&format!("{}", path.display()))
                .unwrap_or(None)
                .map(|cap| {
                    cap.iter()
                        .flatten()
                        .fold(String::new(), |name: String, m: Match| name + m.as_str())
                });
            if name.is_none() {
                warn!("Regex take name failed")
            }
            ScanResult {
                path: result.path,
                name: name.unwrap_or(format!("{}", path.display())),
                size: result.size,
            }
        })
        .collect()
}
