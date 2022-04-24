mod error;
mod arguments;
mod logging;
mod rpc;
mod scanner;
pub use error::{err_msg, Result};
use log::{error, warn, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let client = match rpc::init(opt.host, opt.port, opt.token).await {
        Ok(client) => Some(client),
        Err(e) => {
            error!("Connect to server failed: {}", e);
            if !opt.dry_run {
                return Ok(())
            }
            warn!("Continue because dry-run mode");
            None
        }
    };

    let list = scanner::scan(opt.folder, opt.depth);
    info!("Scan result: {:?}", list);

    client.map(|c| c.report(list));

    if opt.daemon {
        error!("Daemon mod not supportted");
        loop{}
    }
    Ok(())
}