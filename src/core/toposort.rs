use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use crate::{error::Result, utils::topological_sort};

#[derive(structopt::StructOpt)]
pub struct Toposort {
    #[structopt(parse(from_os_str))]
    pub path: Option<std::path::PathBuf>,
    #[structopt(long, short, help = "Group the libraries by updating in paralle")]
    pub plan: bool,
}

impl Toposort {
    pub fn run(&self) -> Result<()> {
        let path = &self
            .path
            .clone()
            .ok_or("File path not provided".to_string())?;
        let contents = read_to_string(path)?;

        let graph: HashMap<String, Vec<String>> = serde_json::from_str(&contents)?;

        // Ascending libs order by dependencies
        let libs_order = topological_sort(&graph);

        if self.plan {
            println!("Upgrade plan:");
            let mut groups = vec![HashSet::new()];

            for lib in libs_order.iter() {
                let deps = graph.get(lib).ok_or("Library not found".to_string())?;
                let libs = merge_sets(&groups);

                if deps.iter().all(|dep| libs.contains(dep)) {
                    let last: &mut HashSet<String> =
                        groups.last_mut().ok_or("There are no groups".to_string())?;
                    last.insert(lib.clone());
                } else {
                    groups.push(HashSet::from_iter([lib.clone()]));
                }
            }

            for (idx, group) in groups.into_iter().enumerate() {
                println!("\nGroup {}:", idx + 1);
                for lib in group {
                    println!("{}", lib);
                }
            }
        } else {
            // Print the topological order
            for lib in libs_order.iter() {
                println!("{}", lib);
            }
        }

        Ok(())
    }
}

fn merge_sets(sets: &[HashSet<String>]) -> HashSet<&String> {
    sets.iter()
        .take(sets.len() - 1)
        .flat_map(|set| set.iter())
        .collect()
}
