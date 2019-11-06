use std::path::PathBuf;

use failure::Error;
use log::info;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about,
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::ColorAuto)
)]
pub struct Args {
    /// The input video file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Where to save the output video
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// Maximum sigma of the gaussian blur
    #[structopt(short, long, default_value = "0.0")]
    pub blur: f64,

    /// Radius of the foveal region in pixels
    #[structopt(short, long, default_value = "32")]
    pub radius: u64,
}

pub fn run(args: Args) -> Result<(), Error> {
    info!("{:?}", args);
    Ok(())
}
