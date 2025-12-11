use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use capitalize::Capitalize;
use regex::Regex;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;
use crate::data_manager::get_app_data;
use crate::git_manager::create_local_repo;
use crate::gui::{BuildData, CreateData};
use crate::template_manager::copy_template;
use crate::util::{error_dialog, warn_dialog};

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

#[derive(Debug)]
pub struct DriftProject {
    pub project_location: PathBuf,
    pub directory_name: String,

    pub package_path: PathBuf,
    pub project_path: PathBuf,
    pub script_path: PathBuf,
    pub build_path: PathBuf,

    pub package_info: PackageInfo,
}

impl DriftProject {
    pub fn new() -> Self {
        Self {
            project_location: PathBuf::new(),
            directory_name: String::new(),

            package_path: PathBuf::new(),
            project_path: PathBuf::new(),
            script_path: PathBuf::new(),
            build_path: PathBuf::new(),

            package_info: PackageInfo::new()
        }
    }

    pub fn has_sufficient_info(&self) -> Result<bool, &'static str> {
        let package_info: &PackageInfo = &self.package_info;

        if package_info.author.is_empty() {
            return Err("Missing author name")
        }

        if package_info.project_name.is_empty() {
            return Err("Missing project name")
        }

        if package_info.version.is_empty() {
            return Err("Missing version number")
        }

        Ok(true)
    }

    pub fn is_creatable(&self) -> Result<bool, &'static str> {
        if self.directory_name.is_empty() {
            return Err("Missing directory name")
        }

        if !self.project_location.exists() {
            return Err("Missing project location")
        }

        // Project with this name already exists
        if self.project_location.join(&self.directory_name).exists() {
            return Err("Project with this name already exists")
        }

        Ok(true)
    }

    pub fn create_project_files(&mut self, create_data: &CreateData) -> Result<(), String> {
        self.project_path = self.project_location.join(&self.directory_name);
        self.build_path =  self.project_path.join("Builds");
        self.script_path = self.project_path.join(&self.package_info.script_name);
        self.package_path = self.script_path.join("package.json");

        // Create directories
        if !self.project_path.exists() {
            if let Err(_) = fs::create_dir_all(&self.project_path) {
                return Err::<(), String>(String::from("Failed to create project path"))
            }
        }
        if let Err(_) = fs::create_dir(&self.build_path) {
            return Err::<(), String>(String::from("Failed to create build directory"))
        }

        if let Err(_) = fs::create_dir(&self.script_path) {
            return Err::<(), String>(String::from("Failed to create script directory"))
        }

        // Create and write package.json
        let package_json_string = serde_json::to_string_pretty(&self.package_info);
        let package_file = File::create(&self.package_path);
        if let Err(_) = package_file {
            return Err::<(), String>(String::from("Failed to create package.json"))
        }
        if let Err(_) = package_file.unwrap().write_all(package_json_string.unwrap().as_bytes()) {
            return Err::<(), String>(String::from("Failed to write to package.json"))
        }

        let template_result = copy_template(&create_data.template, &self.script_path);
        if let Err(e) = template_result {
            return Err::<(), String>(format!("Failed to create script template\n{}", e))
        }

        self.try_write_version();

        if create_data.create_repo {
            if let Err(e) = create_local_repo(&self.project_path) {
                return Err::<(), String>(format!("Failed to create git repo\n{}", e))
            }
        }
        
        if create_data.open_directory {
            let open_result = opener::open(&self.project_path);
            if let Err(_) = open_result {
                return Err::<(), String>(String::from("Failed to open project directory"))
            }
        }

        Ok(())
    }

    pub fn project_from_package(package_info: PackageInfo, package_path: PathBuf) -> Option<Self> {
        if !package_path.exists() {
            return None
        }

        let script_path: PathBuf = package_path.parent().unwrap().to_path_buf();
        if !script_path.exists() {
            return None
        }

        let project_path: PathBuf = script_path.parent().unwrap().to_path_buf();
        if !project_path.exists() {
            return None
        }

        let try_directory_os_string = project_path.file_name();
        if try_directory_os_string.is_none() {
            return None
        }
        let try_directory_string = try_directory_os_string.unwrap().to_str();
        if try_directory_string.is_none() {
            return None
        }
        let directory_name: String = String::from(try_directory_string.unwrap());

        let project_location: PathBuf = project_path.parent().unwrap().to_path_buf();
        if !project_location.exists() {
            return None
        }

        let mut build_path: PathBuf = PathBuf::new();
        let try_build_path: PathBuf = project_path.join("Builds").to_path_buf();
        if try_build_path.exists() {
            build_path = try_build_path;
        }

        // add new keywords from package.json
        let mut app_data = get_app_data().lock().unwrap();
        for keyword in package_info.keywords.clone() {
            if !keyword.is_empty() && app_data.keywords.iter().any(|e| e.to_lowercase() == keyword.to_lowercase()){
                continue;
            }

            // Capitalize to try and match the default keywords
            app_data.keywords.push(keyword.capitalize_first_only());
        }

        Some(Self {
            project_location,
            directory_name,

            package_path,
            project_path,
            script_path,
            build_path,

            package_info
        })
    }

    pub fn save(&self) -> Result<(), String> {
        // Write to package.json
        if !self.package_path.exists() {
            error_dialog("Missing Package", "Failed to find package.json");
            return Err(String::from("Failed to find package.json"))
        }
        fs::write(&self.package_path, serde_json::to_string_pretty(&self.package_info).unwrap()).unwrap();

        // Search for version in main and update it.
        self.try_write_version();

        // Edit script directory name (if applicable)
        if self.script_path.file_name().unwrap().to_str().unwrap() == self.package_info.script_name {
            return Ok(()) // No need to rename directory
        }

        let new_directory = self.script_path.parent().unwrap().join(&self.package_info.script_name);

        if let Err(e) = fs::rename(&self.script_path, &new_directory) {
            warn_dialog("Rename Failure", "Failed to rename script directory to script name");
            return Err(format!("Failed to rename script directory to script name\n{}", e))
        }

        Ok(())
    }

    fn try_write_version(&self) {
        if let Err(_) = self.write_version_to_main() {
            if self.script_path.join("temp_main.luau").exists() {
                if let Err(_) = fs::remove_file(self.script_path.join("temp_main.luau")) {
                    warn_dialog("Version Edit Failure", "Failed to edit version number in main.luau\nFailed to delete temp_main.luau\nBuild will continue");
                } else {
                    warn_dialog("Version Edit Failure", "Failed to edit version number in main.luau\nBuild will continue");
                }
            }
        }
    }

    fn write_version_to_main(&self) -> io::Result<()> {
        let input_file_path = &self.script_path.join("main.luau");
        let output_file_path = &self.script_path.join("temp_main.luau");

        let file = File::open(input_file_path)?;
        let reader = BufReader::new(file);

        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_file_path)?;

        let mut found: bool = false;
        let mut in_next_line: bool = false;
        for line_result in reader.lines() {
            let line = line_result?;

            if !found {
                // Search line for version
                if line.trim() == "--?ScriptVersion" {
                    in_next_line = true;
                    writeln!(output_file, "{}", line)?;
                    continue;
                }

                let pattern: Regex;
                if in_next_line {
                    pattern = Regex::new(r#"^(?P<start>\s*local\s+[^:=]+\s*(?::\s*string\s*)?=\s*["'`])(?P<version>[^"'`]*)(?P<end>["'`]\s*)$"#).unwrap()
                } else {
                    pattern = Regex::new(r#"^(?P<start>\s*local\s+_?version\s*(?::\s*string\s*)?=\s*["'`])(?P<version>[^"'`]*)(?P<end>["'`]\s*)$"#).unwrap()
                }

                if let Some(caps) = pattern.captures(line.as_str()) {
                    found = true;
                    let (start, version, end) = (caps.name("start").unwrap().as_str(), caps.name("version").unwrap().as_str(), caps.name("end").unwrap().as_str());

                    let new_version = if &self.package_info.version == version { version } else { &self.package_info.version };

                    let new_line = String::from(format!("{}{}{}", start, new_version, end));
                    writeln!(output_file, "{}", new_line)?;
                    continue;
                }
            }

            writeln!(output_file, "{}", line)?;
        }

        // overwrite old main.luau
        fs::rename(output_file_path, input_file_path)?;

        Ok(())
    }

    pub fn build(&self, build_data: &BuildData) {
        let mut zip_file_name = self.package_info.script_name.clone();

        if build_data.version_tag {
            zip_file_name = zip_file_name + " - " + self.package_info.version.as_str();
        }

        let zip_file = File::create(&self.build_path.join(zip_file_name + ".zip"));

        if let Err(_) = zip_file {
            error_dialog("Build Failure", "Failed to create zip file");
            return;
        }

        let zip = ZipWriter::new(zip_file.unwrap());
        let zip_result = zip.create_from_directory(&self.script_path);

        if let Err(_) = zip_result {
            error_dialog("Archive Failure", "Failed to archive script directory");
            return;
        }

        if build_data.open_directory {
            let open_result = opener::open(&self.build_path);
            if let Err(_) = open_result {
                warn_dialog("Opening Failure", "Build succeeded but could not open the build directory.\nOpen the directory manually to see the finished build.");
                return;
            }
        }
    }

    pub fn reset_project_data(&mut self) {
        *self = Self::new()
    }
}