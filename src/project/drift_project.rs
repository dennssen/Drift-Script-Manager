use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;
use lazy_static::lazy_static;
use crate::managers::git::create_local_repo;
use crate::gui::state::{BuildData, CreateData};
use crate::managers::template::copy_template;
use crate::project::package_info::PackageInfo;
use crate::utils::dialogs::{error_dialog, warn_dialog};
use crate::utils::error_helper::{json_error_to_io, open_error_to_io, zip_error_to_io};

#[derive(Debug)]
pub struct ProjectPaths {
    pub project_location: PathBuf,
    pub directory_name: String,

    pub package_path: PathBuf,
    pub project_path: PathBuf,
    pub script_path: PathBuf,
    pub build_path: PathBuf,
}

impl ProjectPaths {
    pub fn new() -> Self {
        Self {
            project_location: PathBuf::new(),
            directory_name: String::new(),

            package_path: PathBuf::new(),
            project_path: PathBuf::new(),
            script_path: PathBuf::new(),
            build_path: PathBuf::new(),
        }
    }

    pub fn validate_project_structure(package_path: PathBuf, package_info: &PackageInfo) -> io::Result<ProjectPaths> {
        let mut paths: ProjectPaths = ProjectPaths::new();

        paths.package_path = package_path;
        if !paths.package_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "package.json doesn't exist"))
        }

        let package_name = get_path_file_name(&paths.package_path).unwrap_or("");
        if package_name != "package.json" {
            return Err(Error::new(ErrorKind::InvalidFilename, "file not called package.json"))
        }

        paths.script_path = paths.package_path.parent().unwrap().to_path_buf();
        if !paths.script_path.exists() {
            return Err(Error::new(ErrorKind::InvalidInput, "package.json is not in a script directory"))
        }

        let script_name = get_path_file_name(&paths.script_path).unwrap_or("");
        if script_name != package_info.script_name {
            return Err(Error::new(ErrorKind::InvalidInput, "script directory name does not match package.json 'name' property"))
        }

        paths.project_path = paths.script_path.parent().unwrap().to_path_buf();
        if !paths.project_path.exists() {
            return Err(Error::new(ErrorKind::InvalidInput, "script directory is not in a project directory"))
        }

        paths.directory_name = get_path_file_name(&paths.project_path).unwrap_or("").to_string();
        if paths.directory_name != "Behaviors" {
            return Err(Error::new(ErrorKind::InvalidInput, "'Behaviors' cannot be the project directory. Please create a separate parent directory for your script and try again."))
        }

        paths.project_location = paths.project_path.parent().unwrap().to_path_buf();
        if !paths.project_location.exists() {
            return Err(Error::new(ErrorKind::InvalidInput, "project directory is not in a parent directory"))
        }

        let try_build_path: PathBuf = paths.project_path.join("Builds").to_path_buf();
        if try_build_path.exists() {
            paths.build_path = try_build_path;
        }

        Ok(paths)
    }
}

fn get_path_file_name(path: &Path) -> Option<&str> {
    let os_string = path.file_name()?;
    let string = os_string.to_str()?;

    Some(string)
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

lazy_static! {
    static ref WILDCARD_VERSION_PATTERN: Regex =Regex::new(r#"^(?P<start>\s*local\s+[^:=]+\s*(?::\s*string\s*)?=\s*["'`])(?P<version>[^"'`]*)(?P<end>["'`]\s*)$"#).unwrap();
    static ref VERSION_PATTERN: Regex = Regex::new(r#"^(?P<start>\s*local\s+_?version\s*(?::\s*string\s*)?=\s*["'`])(?P<version>[^"'`]*)(?P<end>["'`]\s*)$"#).unwrap();
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

    pub fn has_sufficient_info(&self) -> Result<(), &'static str> {
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

        Ok(())
    }

    pub fn is_creatable(&self) -> Result<(), &'static str> {
        if self.directory_name.is_empty() {
            return Err("Missing directory name")
        }

        if !self.project_location.exists() {
            return Err("Missing project location")
        }

        if self.project_location.join(&self.directory_name).exists() {
            return Err("Project with this name already exists")
        }

        Ok(())
    }

    pub fn create_project_files(&mut self, create_data: &CreateData) -> io::Result<()> {
        self.project_path = self.project_location.join(&self.directory_name);
        self.build_path = self.project_path.join("Builds");
        self.script_path = self.project_path.join(&self.package_info.script_name);
        self.package_path = self.script_path.join("package.json");

        // Create directories
        if self.project_path.exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, "Project directory already exists"))
        }

        fs::create_dir_all(&self.project_path)?;

        fs::create_dir(&self.build_path)?;
        fs::create_dir(&self.script_path)?;

        // Create and write package.json
        let package_json_string = serde_json::to_string_pretty(&self.package_info)?;
        let mut package_file = File::create(&self.package_path)?;

        package_file.write_all(package_json_string.as_bytes())?;

        copy_template(&create_data.template, &self.script_path)?;

        self.try_write_version();

        if create_data.create_repo {
            if let Err(e) = create_local_repo(&self.project_path) {
                return Err(e)
            }
        }

        if create_data.open_directory {
            let open_result = opener::open(&self.project_path);
            if let Err(e) = open_result {
                let e = open_error_to_io(&e);
                return Err(e)
            }
        }

        Ok(())
    }


    pub fn project_from_package(package_info: PackageInfo, paths: ProjectPaths) -> Self {
        Self {
            project_location: paths.project_location,
            directory_name: paths.directory_name,

            package_path: paths.package_path,
            project_path: paths.project_path,
            script_path: paths.script_path,
            build_path: paths.build_path,

            package_info
        }
    }

    pub fn save(&self) -> Result<(), ()> {
        // Write to package.json
        if !self.package_path.exists() {
            let e: Error = Error::new(ErrorKind::NotFound, "Project is missing a package.json path");
            error_dialog("Missing Package", "Failed to find package.json", &e);
            return Err(())
        }

        let json = serde_json::to_string_pretty(&self.package_info);

        if let Err(e) = json {
            let e = json_error_to_io(&e);
            error_dialog("Serialization Failure", "Failed to serialize package info", &e);
            return Err(())
        }

        let result = fs::write(&self.package_path, json.unwrap());
        if let Err(e) = result {
            error_dialog("Write Failure", "Failed to write to package.json", &e);
            return Err(())
        }

        // Search for version in main and update it.
        self.try_write_version();

        // Edit script directory name (if applicable)
        if self.script_path.file_name().unwrap().to_str().unwrap() == self.package_info.script_name {
            return Ok(()) // No need to rename directory
        }

        let new_directory = self.script_path.parent().unwrap().join(&self.package_info.script_name);

        if let Err(_) = fs::rename(&self.script_path, &new_directory) {
            warn_dialog("Rename Failure", "Failed to rename script directory to script name");
            return Err(())
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

                let pattern: &Regex;
                if in_next_line {
                    pattern = &*WILDCARD_VERSION_PATTERN;
                } else {
                    pattern = &*VERSION_PATTERN;
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

        if let Err(e) = zip_file {
            error_dialog("Build Failure", "Failed to create zip file", &e);
            return;
        }

        let zip = ZipWriter::new(zip_file.unwrap());
        let zip_result = zip.create_from_directory(&self.script_path);

        if let Err(e) = zip_result {
            let e = zip_error_to_io(&e);
            error_dialog("Archive Failure", "Failed to archive script directory", &e);
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    use std::fs;
    use crate::managers::template::{EmbeddedTemplate, Template};

    fn script_name(author_name: &str, project_name: &str) -> String {
        format!("{}.{}", author_name.to_lowercase().replace(" ", ""), project_name.to_lowercase().replace(" ", ""))
    }

    fn create_test_package(author_name: &str, project_name: &str) -> PackageInfo {
        let script_name = script_name(author_name, project_name);

        PackageInfo {
            author: author_name.to_string(),
            project_name: project_name.to_string(),
            script_name,
            version: "1.0.0".to_string(),
            description: String::new(),
            keywords: Vec::new(),
            default_keybind: String::new(),
            main: "main.luau".to_string()
        }
    }

    struct TestProject {
        _temp: TempDir,
        pub project_path: PathBuf,
        pub package_path: PathBuf,
        pub build_path: Option<PathBuf>,
    }

    impl TestProject {
        fn valid(author_name: &str, project_name: &str) -> Self {
            let temp = tempdir().unwrap();
            let project_location = temp.path();
            let project_path = project_location.join(project_name);
            let script_path = project_path.join(script_name(author_name, project_name));

            fs::create_dir_all(&script_path).unwrap();

            let package_path = script_path.join("package.json");
            fs::write(&package_path, "{}").unwrap();

            Self {
                _temp: temp,
                project_path,
                package_path,
                build_path: None,
            }
        }

        fn with_builds(mut self) -> Self {
            let build_path = self.project_path.join("Builds");
            fs::create_dir(&build_path).unwrap();
            self.build_path = Some(build_path);
            self
        }

        fn invalid() -> Self {
            let temp = tempdir().unwrap();
            let project_location = temp.path();
            let project_path = project_location.join("Project");

            fs::create_dir_all(&project_path).unwrap();

            let package_path = project_path.join("package.json");
            fs::write(&package_path, "{}").unwrap();

            Self {
                _temp: temp,
                project_path,
                package_path,
                build_path: None,
            }
        }
    }

    #[test]
    fn test_validate_project_structure_with_builds() {
        let author_name = "Me";
        let project_name = "Project";
        let package_info = create_test_package(author_name, project_name);

        let test_project: TestProject = TestProject::valid(&package_info.author, &package_info.project_name).with_builds();

        let result = ProjectPaths::validate_project_structure(test_project.package_path, &package_info);

        assert!(result.is_ok(), "Result should be Ok");

        let result = result.unwrap();
        assert_eq!(result.directory_name, project_name, "Directory name should be project name");
        assert!(result.build_path.exists(), "Builds path should exist");
    }

    #[test]
    fn test_validate_project_structure_without_builds() {
        let author_name = "Me";
        let project_name = "Project";
        let package_info = create_test_package(author_name, project_name);

        let test_project: TestProject = TestProject::valid(&package_info.author, &package_info.project_name);

        let result = ProjectPaths::validate_project_structure(test_project.package_path, &package_info);

        assert!(result.is_ok(), "Result should be Ok");

        let result = result.unwrap();
        assert!(!result.build_path.exists(), "Builds path should not exist");
    }

    #[test]
    fn test_invalid_project_structure_without_builds() {
        let author_name = "Me";
        let project_name = "Project";
        let package_info = create_test_package(author_name, project_name);

        let test_project: TestProject = TestProject::invalid();

        let result = ProjectPaths::validate_project_structure(test_project.package_path, &package_info);

        assert!(result.is_err());
    }

    #[test]
    fn test_has_sufficient_info() {
        let mut project: DriftProject = DriftProject::new();
        project.package_info.author = "Me".to_string();
        project.package_info.project_name = "Project".to_string();
        project.package_info.version = "1.0.0".to_string();

        assert!(project.has_sufficient_info().is_ok())
    }

    #[test]
    fn test_has_insufficient_info() {
        let mut project: DriftProject = DriftProject::new();
        project.package_info.project_name = "Project".to_string();
        project.package_info.version = "1.0.0".to_string();

        assert_eq!(project.has_sufficient_info(), Err("Missing author name"));
    }

    #[test]
    fn test_is_creatable() {
        let temp = tempdir().unwrap();
        let mut project: DriftProject = DriftProject::new();
        project.directory_name = "Project".to_string();
        project.project_location = temp.path().to_path_buf();

        assert!(project.is_creatable().is_ok());
    }

    #[test]
    fn test_is_creatable_already_exists() {
        let temp = tempdir().unwrap();
        fs::create_dir(temp.path().join("Project")).unwrap();

        let mut project: DriftProject = DriftProject::new();
        project.directory_name = "Project".to_string();
        project.project_location = temp.path().to_path_buf();

        assert_eq!(project.is_creatable(), Err("Project with this name already exists"))
    }

    #[test]
    fn test_create_project_files_default() {
        let temp = tempdir().unwrap();

        let mut project: DriftProject = DriftProject::new();
        project.project_location = temp.path().to_path_buf();
        project.directory_name = "Project".to_string();
        let package_info = &mut project.package_info;
        package_info.project_name = "Project".to_string();
        package_info.version = "1.0.0".to_string();
        package_info.author = "Me".to_string();
        package_info.script_name = script_name("Me", "Project");
        package_info.main = "main.luau".to_string();

        let create_data: CreateData = CreateData {
            open_directory: false,
            create_repo: false,
            template: Template::Embedded(EmbeddedTemplate::Default)
        };



        assert!(project.create_project_files(&create_data).is_ok());
        assert!(project.project_path.exists());
        assert!(project.build_path.exists());
        assert!(project.script_path.exists());
        assert!(project.package_path.exists());
    }
}