use structopt::StructOpt;

use crate::error::Result;

mod generate;
mod list;
mod models;
mod toposort;

#[derive(StructOpt)]
#[structopt(name = "nestlib")]
pub enum Opt {
    #[structopt(name = "generate", about = "Generate the dependecy graph")]
    Generate(generate::Generate),
    #[structopt(
        name = "toposort",
        about = "Topological Sorting on the given dependency graph"
    )]
    Toposort(toposort::Toposort),
    #[structopt(name = "list", about = "List packages and versions")]
    List(list::List),
}

impl Opt {
    pub fn run(&self) -> Result<()> {
        match self {
            Opt::Generate(generate) => {
                generate.run()?;
                Ok(())
            }
            Opt::Toposort(toposort) => {
                toposort.run()?;
                Ok(())
            }
            Opt::List(list) => {
                list.run()?;
                Ok(())
            }
        }
    }
}
