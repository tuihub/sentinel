mod arguments;
mod rpc;

use std::fs::File;

use log::error;
use sentinel::{Result, __private::logging};

use crate::arguments::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let mut client = match rpc::init(opt.host, opt.port, opt.token).await {
        Ok(client) => client,
        Err(e) => {
            error!("Connect to server failed: {}", e);
            return Ok(());
        }
    };

    match opt.cmd {
        Command::Upload { path } => {
            let f = File::open(&path)?;
            let upload_token = client
                .upload_image(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    f.metadata()?.len() as i64,
                )
                .await?;
            client.upload_file(f, upload_token).await?;
        }
    };

    Ok(())
}
