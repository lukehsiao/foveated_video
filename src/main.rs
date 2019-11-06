use failure::Error;
use pretty_env_logger;
use structopt::StructOpt;

use foveated_video::{run, Args};

fn main() -> Result<(), Error> {
    pretty_env_logger::init_timed();

    let args = Args::from_args();

    run(args)
}
