use std::{fs, io};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use crate::utils::dialogs::error_dialog;
use crate::utils::error_helper::json_error_to_io;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    pub author: String,
    #[serde(rename = "displayName")]
    pub project_name: String,
    #[serde(rename = "name")]
    pub script_name: String,
    pub version: String,
    pub description: String,
    pub keywords: Vec<String>,
    #[serde(rename = "defaultKeybind")]
    pub default_keybind: String,
    pub main: String,
}

impl PackageInfo {
    pub fn new() -> Self {
        Self {
            author: String::new(),
            project_name: String::new(),
            script_name: String::new(),
            version: String::new(),
            description: String::new(),
            keywords: Vec::new(),
            main: String::from("main.luau"),
            default_keybind: String::new()
        }
    }

    pub fn get_package_file() -> Result<(PackageInfo, PathBuf), Error> {
        let package_path = FileDialog::new()
            .add_filter("Script package", &["json"])
            .set_title("Open the projects package.json")
            .pick_file();

        PackageInfo::package_info_from_file(package_path)
    }
    fn package_info_from_file(package_path: Option<PathBuf>) -> Result<(PackageInfo, PathBuf), Error> {
        if package_path.is_none() {
            return Err(Error::new(ErrorKind::NotFound, "No path specified"))
        }
    
        let package_path: PathBuf = package_path.unwrap();
    
        if !package_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "Could not find package.json"))
        }
    
        let read_result = fs::read_to_string(package_path.clone())?;
    
        let try_parse: Result<PackageInfo, _> = serde_json::from_str(&read_result);
        if let Err(e) = try_parse {
            let e = json_error_to_io(&e);
            return Err(e)
        }
    
        Ok((try_parse?, package_path))
    }
}


#[cfg(test)]
mod tests {
    use std::fs::write;
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_package_info_from_file() {
        let temp = tempdir().unwrap();
        let test_package = PackageInfo {
            author: "Me".to_string(),
            project_name: "Project".to_string(),
            script_name: "me.project".to_string(),
            version: "1.0.0".to_string(),
            description: String::new(),
            keywords: Vec::new(),
            default_keybind: String::new(),
            main: "main.luau".to_string(),
        };
        let test_package_path = temp.path().join("package.json");
        write(&test_package_path, serde_json::to_string(&test_package).unwrap()).unwrap();

        let result = PackageInfo::package_info_from_file(Some(test_package_path));
        assert!(result.is_ok());
        
        let (package, _) = result.unwrap();

        assert_eq!(package.author, test_package.author);
        assert_eq!(package.project_name, test_package.project_name);
        assert_eq!(package.script_name, test_package.script_name);
        assert_eq!(package.version, test_package.version);
    }
}