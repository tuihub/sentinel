mod arguments;

use sentinel::{Result, __private::logging, process_tracing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let (start_time, end_time, exit_code) = process_tracing::start_and_trace(
        opt.trace_mode,
        &opt.exe_name,
        opt.exe_path.as_path(),
        opt.mon_path.as_path(),
        opt.working_dir.as_path(),
        100,
        1000,
    )?;
    println!(
        "start_time: {}, end_time: {}, exit_code: {}",
        start_time, end_time, exit_code
    );
    Ok(())
}
