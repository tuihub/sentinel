use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "savedata-cli")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(short, long, parse(from_os_str))]
    pub game_root: PathBuf,

    #[structopt(long, parse(from_os_str))]
    pub user_document: Option<PathBuf>,

    #[structopt(long, parse(from_os_str))]
    pub user_home: Option<PathBuf>,
}

#[derive(StructOpt)]
#[structopt(about = "the stupid content tracker")]
pub enum Command {
    Store {
        #[structopt(short, long, parse(from_os_str))]
        save_data: PathBuf,

        #[structopt(short, long, parse(from_os_str))]
        config_file: PathBuf,
    },
    Restore {
        #[structopt(short, long, parse(from_os_str))]
        save_data: PathBuf,
    },
}

/// Return command arguments
pub fn get_opt() -> Opt {
    Opt::from_args()
}
