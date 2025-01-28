use std::collections::HashMap;

use serde_json::to_string_pretty;
use structopt::StructOpt;

use crate::{error::Result, utils::read_dir_contents};

use super::models::Package;

#[derive(Debug, StructOpt)]
pub struct Generate {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
    #[structopt(long)]
    json: bool,
    #[structopt(
        long,
        short,
        help = "Provide a pattern to filter libraries",
        default_value = ""
    )]
    pattern: String,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().ok_or("Path not provided".to_string())?;
        let paths = read_dir_contents(&path)?;
        let package_map = paths
            .into_iter()
            .filter_map(|path| Package::from_file(&path).ok())
            .filter_map(|package| {
                let dependencies: Vec<_> = package
                    .dependencies
                    .into_iter()
                    .filter(|dep| dep.contains(&self.pattern))
                    .collect();

                if package.name.contains(&self.pattern) || !dependencies.is_empty() {
                    Some((package.name, dependencies))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if self.json {
            println!("{}", to_string_pretty(&package_map)?);
        }

        Ok(())
    }
}
