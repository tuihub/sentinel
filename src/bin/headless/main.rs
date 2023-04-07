mod arguments;
mod logging;
mod rpc;

use log::{error, info, warn};
use sentinel::scanner;
use sentinel::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let client = match rpc::init(opt.host, opt.port, opt.token).await {
        Ok(client) => Some(client),
        Err(e) => {
            error!("Connect to server failed: {}", e);
            if !opt.dry_run {
                return Ok(());
            }
            warn!("Continue because dry-run mode");
            None
        }
    };

    info!("Scan Mode: {}", opt.scan_mode);
    let list = match opt.scan_mode.as_str() {
        arguments::MODE_SINGLE => scanner::single_file_mode(opt.folder, opt.depth),
        arguments::MODE_FIXED => scanner::fixed_depth_mode(opt.folder, opt.depth),
        arguments::MODE_FILES => scanner::files_folder_mode(opt.folder, opt.depth),
        _ => {
            error!("Unsupported scan mode");
            return Ok(());
        }
    };
    info!("Scan result: {:?}", list);

    if let Some(c) = client {
        c.report(list).await?;
    }

    if opt.daemon {
        error!("Daemon mod not supported");
    }
    Ok(())
}
