use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use crate::{error::Result, utils::topological_sort};

#[derive(structopt::StructOpt)]
pub struct Toposort {
    #[structopt(parse(from_os_str))]
    pub path: Option<std::path::PathBuf>,
}

impl Toposort {
    pub fn run(&self) -> Result<()> {
        let path = &self
            .path
            .clone()
            .ok_or("File path not provided".to_string())?;
        let contents = read_to_string(&path)?;

        let graph: HashMap<String, Vec<String>> = serde_json::from_str(&contents)?;
        // Ascending libs order by dependencies
        let libs_order = topological_sort(&graph);

        for lib in libs_order {
            let deps = graph.get(&lib).unwrap();
            println!("{} {:?}", lib, deps);
        }

        Ok(())
    }
}
