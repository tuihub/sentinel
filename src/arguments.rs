use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "sentinel")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(short, long)]
    pub token: String,

    #[structopt(short, long, parse(from_os_str))]
    pub folder: PathBuf,

    #[structopt(long, default_value = "1")]
    pub depth: usize,

    #[structopt(short, long)]
    pub host: String,

    #[structopt(short, long)]
    pub port: String,

    /// Only connect to server and scan folder
    #[structopt(long)]
    pub dry_run: bool,

    #[structopt(short)]
    pub daemon: bool,
}

/// Return command arguments
pub fn get_opt() -> Opt {
    Opt::from_args()
}
