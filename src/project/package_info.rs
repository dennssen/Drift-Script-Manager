use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use crate::utils::dialogs::error_dialog;

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

    pub fn get_package_file() -> Result<(PackageInfo, PathBuf), ()> {
        let package_path = FileDialog::new()
            .add_filter("Script package", &["json"])
            .set_title("Open the projects package.json")
            .pick_file();

        if package_path.is_none() {
            return Err(())
        }

        let package_path: PathBuf = package_path.unwrap();

        let read_result = fs::read_to_string(package_path.clone());
        if let Err(_) = read_result {
            error_dialog("Read Failure", "Failed to read from package.json.");

            return Err(())
        }

        let try_parse: Result<PackageInfo, _> = serde_json::from_str(&read_result.unwrap());
        if let Err(_) = try_parse {
            error_dialog("Parse Failure", "Failed to parse package.json.\nMake sure to select the correct json file.");

            return Err(())
        }

        Ok((try_parse.unwrap(), package_path))
    }
}