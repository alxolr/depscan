use core::Opt;

use error::Result;
use structopt::StructOpt;

mod core;
mod error;
mod utils;

fn start_cli() -> Result<()> {
    let opt = Opt::from_args();
    opt.run()?;

    // let toposort = Toposort {
    //     path: Some(PathBuf::from("./deps.json")),
    //     plan: true,
    // };

    // toposort.run()?;

    Ok(())
}

fn main() -> Result<()> {
    start_cli()?;

    Ok(())
}
