use core::Opt;

use error::Result;
use structopt::StructOpt;

mod core;
mod error;

fn start_cli() -> Result<()> {
    let opt = Opt::from_args();
    opt.run()?;

    Ok(())
}

fn main() -> Result<()> {
    start_cli()?;

    Ok(())
}
