use crate::{error::Result, utils::read_dir_contents};

use super::models::Package;

#[derive(structopt::StructOpt)]
#[structopt(about = "List nodejs packages and versions")]
pub struct List {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

impl List {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().ok_or("Path not provided".to_string())?;
        let paths = read_dir_contents(&path)?;

        let packages: Vec<Package> = paths
            .into_iter()
            .filter_map(|path| Package::from_file(&path).ok())
            .collect();

        for package in packages {
            println!("{}@{}", package.name, package.version);
        }

        Ok(())
    }
}
