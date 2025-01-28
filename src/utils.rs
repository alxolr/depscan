use std::{fs, path::PathBuf};

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
