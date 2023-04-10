mod arguments;

use std::{fs, io::Read, path::PathBuf};

use log::error;
use sentinel::{
    Result,
    __private::logging,
    save_data::{restore, store, CommonPath},
};

use crate::arguments::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let user_document: PathBuf = match dirs::document_dir() {
        Some(path) if path.is_dir() => path,
        _ => match opt.user_document {
            Some(path) if path.is_dir() => path,
            _ => {
                error!("get user document directory failed");
                return Ok(());
            }
        },
    };
    let user_profile: PathBuf = match dirs::home_dir() {
        Some(path) if path.is_dir() => path,
        _ => match opt.user_home {
            Some(path) if path.is_dir() => path,
            _ => {
                error!("get user home directory failed");
                return Ok(());
            }
        },
    };
    let common_path = CommonPath {
        document: user_document,
        profile: user_profile,
        game: opt.game_root,
    };

    match opt.cmd {
        Command::Store {
            save_data,
            config_file,
        } => {
            let mut config = match fs::File::open(config_file) {
                Ok(f) => f,
                Err(err) => {
                    error!("open config file failed: {:?}", err);
                    return Ok(());
                }
            };
            let mut config_str = String::new();
            if let Err(err) = config.read_to_string(&mut config_str) {
                error!("read config failed {:?}", err)
            };
            match store(common_path, save_data, config_str) {
                Err(err) => error!("store failed: {:?}", err),
                Ok(_) => println!("store finished"),
            };
        }
        Command::Restore { save_data } => {
            match restore(common_path, save_data) {
                Err(err) => error!("restore failed: {:?}", err),
                Ok(_) => println!("restore finished"),
            };
        }
    }

    Ok(())
}
