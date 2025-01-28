use structopt::StructOpt;

use crate::error::Result;

mod generate;
mod models;

#[derive(Debug, StructOpt)]
#[structopt(name = "nestlib")]
pub enum Opt {
    #[structopt(name = "generate", about = "Generate the dependecy graph")]
    Generate(generate::Generate),
}

impl Opt {
    pub fn run(&self) -> Result<()> {
        match self {
            Opt::Generate(generate) => {
                generate.run()?;
                Ok(())
            }
        }
    }
}
