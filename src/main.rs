use core::{
    toposort::{self, Toposort},
    Opt,
};
use std::path::PathBuf;

use error::Result;
use structopt::StructOpt;

mod core;
mod error;
mod utils;

fn start_cli() -> Result<()> {
    let opt = Opt::from_args();
    opt.run()?;

    Ok(())
}

fn main() -> Result<()> {
    start_cli()?;

    Ok(())
}
