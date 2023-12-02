use std::path::PathBuf;

use sentinel::process_tracing::TraceMode;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "savedata-cli")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(long, parse(from_os_str))]
    pub exe_path: PathBuf,

    #[structopt(long)]
    pub exe_name: String,

    #[structopt(long, parse(from_os_str))]
    pub mon_path: PathBuf,

    #[structopt(long, parse(from_os_str))]
    pub working_dir: PathBuf,

    #[structopt(long)]
    pub trace_mode: TraceMode,
}

/// Return command arguments
pub fn get_opt() -> Opt {
    Opt::from_args()
}
