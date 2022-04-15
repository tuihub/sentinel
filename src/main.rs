mod error;
mod arguments;
mod logging;
mod rpc;
pub use error::{err_msg, Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    rpc::init(opt.host, opt.port, opt.token).await?;

    Ok(())
}