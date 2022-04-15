use std::{path::PathBuf};

use log::info;
use walkdir::WalkDir;


pub fn scan(folder: PathBuf, depth: usize) -> Vec<String> {
    WalkDir::new(folder)
        .min_depth(depth)
        .max_depth(depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().to_str()
            .map(|file| file.to_owned())
        )
        .collect()
}

fn calculate_size(folder: PathBuf) -> u64 {
    if !folder.is_dir() {
        return 0
    }
    let total_size = WalkDir::new(folder)
        .min_depth(1)
        .max_depth(20)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    info!("Total size: {} bytes.", total_size);
    total_size
}
