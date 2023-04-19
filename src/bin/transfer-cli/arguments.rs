use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "transfer-cli")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(short, long)]
    pub host: String,

    #[structopt(short, long)]
    pub port: String,

    #[structopt(short, long)]
    pub token: String,
}

#[derive(StructOpt)]
#[structopt(about = "the stupid content tracker")]
pub enum Command {
    Upload {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,
    },
}

/// Return command arguments
pub fn get_opt() -> Opt {
    Opt::from_args()
}
