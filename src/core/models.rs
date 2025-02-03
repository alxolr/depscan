use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::Deserialize;

use crate::error::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
}

impl Package {
    pub fn from_file(path: &str) -> Result<Package> {
        let contents = std::fs::read_to_string(path)?; // this is a json file
        let package: PackageJson = serde_json::from_str(&contents)?;

        let dependencies: Vec<String> = merge_hashmaps(vec![
            package.dependencies,
            package.dev_dependencies,
            package.peer_dependencies,
        ])
        .into_iter()
        .map(|(name, _)| name)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

        Ok(Package {
            name: package.name,
            version: package.version,
            dependencies,
        })
    }
}

fn merge_hashmaps<K, V>(maps: Vec<Option<HashMap<K, V>>>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    maps.into_iter()
        .flatten()
        .flat_map(|map| map.into_iter())
        .collect()
}
