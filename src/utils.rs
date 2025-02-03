use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::PathBuf,
};

use crate::error::Result;

///
/// Find all the package.json files in the given directory
/// and its subdirectories
pub fn read_dir_contents(directory_path: &PathBuf) -> Result<Vec<String>> {
    let exclude_dirs = ["node_modules", ".git", ".vscode", "dist"];

    let paths = fs::read_dir(directory_path)?;

    let package_paths = paths
        .into_iter()
        .flat_map(|entry| {
            let entry = entry?;

            if entry.file_type()?.is_dir() {
                let path = entry.path();
                let path_str = entry
                    .path()
                    .as_path()
                    .as_os_str()
                    .to_str()
                    .ok_or("Invalid Path".to_string())?
                    .to_string();

                if exclude_dirs.iter().all(|dir| !path_str.contains(dir)) {
                    read_dir_contents(&path)
                } else {
                    Ok(vec![])
                }
            } else {
                let path_str = entry
                    .path()
                    .as_path()
                    .as_os_str()
                    .to_str()
                    .ok_or("Invalid Path".to_string())?
                    .to_string();

                if path_str.ends_with("package.json") {
                    Ok(vec![path_str.to_string()])
                } else {
                    Ok(vec![])
                }
            }
        })
        .flatten()
        .collect();

    Ok(package_paths)
}

pub fn topological_sort(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let libs: Vec<String> = graph.clone().into_keys().collect();
    let mut sorted_libs = Vec::with_capacity(libs.len());
    let direct_graph = graph.clone();
    let indirect_graph = graph.iter().fold(
        HashMap::<&String, Vec<String>>::new(),
        |mut acc, (key, deps)| {
            for dep in deps {
                acc.entry(dep).or_default().push(key.clone());
            }

            acc
        },
    );

    let mut dependency_count = HashMap::new();

    for lib in libs.iter() {
        let count = direct_graph.get(lib).map_or(0, |parents| {
            parents
                .iter()
                .filter(|parent| libs.contains(parent))
                .count()
        });
        dependency_count.insert(lib, count);
    }

    let mut queue: VecDeque<_> = dependency_count
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&lib, _)| lib)
        .collect();

    while let Some(lib) = queue.pop_front() {
        sorted_libs.push(lib.to_string());

        if let Some(children) = indirect_graph.get(lib) {
            for child in children.iter().filter(|child| libs.contains(child)) {
                if let Some(count) = dependency_count.get_mut(&child) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
    }

    sorted_libs
}
