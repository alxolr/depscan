use structopt::StructOpt;

use crate::{error::Result, utils::read_dir_contents};

use super::models::Package;

#[derive(Debug, StructOpt)]
pub struct Generate {
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().ok_or("Path not provided".to_string())?;
        let paths = read_dir_contents(&path)?;

        for path in paths {
            let package = Package::from_file(&path)?;
            println!("{:?}", package);
        }

        Ok(())
    }
}
