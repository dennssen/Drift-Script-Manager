use std::fs::{create_dir, create_dir_all, read_dir};
use std::{fs, io};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use include_dir::include_dir;
use crate::gui::state::CreateTemplateData;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Template {
    Embedded(EmbeddedTemplate),
    Custom(String)
}

impl Template {
    pub fn name(&self) -> String {
        match self {
            Self::Embedded(t) => t.name().to_string(),
            Self::Custom(name) => name.clone()
        }
    }

    pub fn has_sufficient_info(template_name: &String, existing_templates: &Vec<Template>) -> io::Result<()> {
        if let Err(e) = Self::has_valid_name(template_name) {
            return Err(e)
        }

        if let Err(e) = Self::is_unique(template_name, existing_templates) {
            return Err(e)
        }

        Ok(())
    }

    fn is_unique(template_name: &String, existing_templates: &Vec<Template>) -> io::Result<()> {
        for template in existing_templates {
            let name = template.name();
            if name == *template_name {
                return Err(Error::new(ErrorKind::AlreadyExists, "A template with this name already exists"));
            }
        }

        Ok(())
    }

    fn has_valid_name(template_name: &String) -> io::Result<()> {
        if template_name.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Template name cannot be empty"));
        }

        if template_name.replace(" ", "").is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Template name cannot consist of whitespace only"));
        }

        Ok(())
    }
    
    pub fn create_custom_template(create_data: &CreateTemplateData, existing_templates: &Vec<Template>) -> io::Result<(PathBuf, Option<String>)> {
        if let Err(e) = Self::has_sufficient_info(&create_data.template_name, existing_templates) {
            return Err(e)
        }

        let templates_dir = get_custom_templates_dir()?;

        Self::create_custom_template_files(&templates_dir, &create_data.template_name, create_data.create_main)
    }

    fn create_custom_template_files(template_location: &PathBuf, template_name: &String, create_main: bool) -> io::Result<(PathBuf, Option<String>)> {
        let new_template_path = template_location.join(template_name);
        create_dir(&new_template_path)?;

        let mut warning = None;
        if create_main {
            if let Err(e) = fs::write(new_template_path.join("main.luau"), "") {
                warning = Some(format!("Failed to create main.luau.\nContinuing anyways.\nError: {}", e))
            }
        }

        Ok((new_template_path, warning))
    }

    pub fn edit_custom_template(&self, new_template_info: Template) -> io::Result<Vec<Template>> {
        if let Template::Embedded(_) = self {
            return Err(Error::new(ErrorKind::InvalidInput, "Cannot Edit an Embedded template"))
        }

        let template_dir = get_custom_templates_dir()?;

        Self::edit_custom_template_files(&template_dir, self.name(), new_template_info.name())?;

        get_custom_templates()
    }

    fn edit_custom_template_files(template_location: &PathBuf, old_template_name: String, new_template_name: String) -> io::Result<PathBuf> {
        let old_template_path = template_location.join(old_template_name);
        let new_template_path = template_location.join(new_template_name);

        fs::rename(&old_template_path, &new_template_path)?;

        Ok(new_template_path)
    }

    pub fn delete_custom_template(&self) -> io::Result<Vec<Template>> {
        if let Template::Embedded(_) = self {
            return Err(Error::new(ErrorKind::InvalidInput, "Cannot Delete an Embedded template"))
        }

        let template_dir = get_custom_templates_dir()?;

        Self::delete_custom_template_files(&template_dir, self.name())?;

        get_custom_templates()
    }

    fn delete_custom_template_files(template_location: &PathBuf, template_name: String) -> io::Result<()> {
        let template_path = template_location.join(template_name);

        fs::remove_dir_all(template_path)?;

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum EmbeddedTemplate {
    Empty,
    Default,
}

impl EmbeddedTemplate {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Default => "default"
        }
    }

    pub fn all() -> &'static [EmbeddedTemplate] {
        &[Self::Empty, Self::Default]
    }
}

const TEMPLATES_DIR: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/templates");

pub fn get_custom_templates_dir() -> io::Result<PathBuf> {
    let path = dirs::document_dir().unwrap().join("DriftScriptManager").join("templates");
    if !path.exists() {
        create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn get_custom_templates() -> io::Result<Vec<Template>> {
    let templates_dir = get_custom_templates_dir()?;

    let mut templates = Vec::new();

    for entry in read_dir(templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    templates.push(Template::Custom(name_str.to_string()));
                }
            }
        }
    }

    templates.sort();
    Ok(templates)
}

pub fn copy_template(template: &Template, script_path: &Path) -> io::Result<()> {
    match template {
        Template::Embedded(t) => copy_embedded_template(t, script_path),
        Template::Custom(name) => copy_custom_template(name, script_path),
    }
}

pub fn copy_embedded_template(template: &EmbeddedTemplate, script_path: &Path) -> io::Result<()> {
    let template_dir = TEMPLATES_DIR
        .get_dir(template.name())
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Template not found"))?;

    copy_embedded_dir_recursive(&template_dir, &script_path)?;

    Ok(())
}


fn copy_embedded_dir_recursive(dir: &include_dir::Dir, dst: &Path) -> io::Result<()> {
    for file in dir.files() {
        let file_path = dst.join(file.path().file_name().unwrap());

        fs::write(&file_path, file.contents())?;
    }


    for dir in dir.dirs() {
        let dir_path = dst.join(dir.path().file_name().unwrap());

        create_dir_all(&dir_path)?;

        copy_embedded_dir_recursive(dir, &dir_path)?;
    }

    Ok(())
}

fn copy_custom_template(template_name: &str, script_path: &Path) -> io::Result<()> {
    let template_dir = get_custom_templates_dir()?.join(template_name);

    if template_name.is_empty() || !template_dir.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Custom template not found"));
    }

    copy_dir_recursive(&template_dir, script_path)?;

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if path.is_dir() {
            create_dir_all(&dst_path)?;
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, write};
    use tempfile::tempdir;
    use super::*;

    fn is_directory_empty<P: AsRef<Path>>(path: P) -> io::Result<bool> {
        let mut entries = read_dir(path)?;

        Ok(entries.next().is_none())
    }

    #[test]
    fn test_copy_embedded_template() {
        let temp = tempdir().unwrap();
        let template = EmbeddedTemplate::Default;

        assert!(copy_embedded_template(&template, temp.path()).is_ok());
        assert!(!is_directory_empty(temp.path()).unwrap());
    }

    #[test]
    fn test_copy_dir_recursive() {
        let temp = tempdir().unwrap();
        let src_path = temp.path().join("src");
        let dst_path = temp.path().join("dst");

        create_dir(&src_path).unwrap();
        create_dir(&dst_path).unwrap();

        write(src_path.join("test.txt"), "").unwrap();

        assert!(copy_dir_recursive(&src_path, &dst_path).is_ok());
        assert!(!is_directory_empty(dst_path).unwrap());
    }

    #[test]
    fn test_is_not_unique() {
        let template_name: String = String::from("not unique");
        let existing_templates: Vec<Template> = vec![Template::Custom("not unique".to_string())];

        assert!(Template::is_unique(&template_name, &existing_templates).is_err())
    }

    #[test]
    fn test_has_invalid_name_empty() {
        assert!(Template::has_valid_name(&String::new()).is_err())
    }

    #[test]
    fn test_has_invalid_name_whitespace() {
        assert!(Template::has_valid_name(&String::from(" ")).is_err())
    }

    #[test]
    fn test_has_valid_name() {
        assert!(Template::has_valid_name(&String::from("Valid")).is_ok())
    }

    #[test]
    fn test_create_custom_template_files() {
        let temp = tempdir().unwrap();
        let template_name = "template";
        let create_main = true;

        let result = Template::create_custom_template_files(&temp.path().to_path_buf(), &template_name.to_string(), create_main);

        assert!(result.is_ok());
        let (path, _) = result.unwrap();
        assert!(path.exists());
        assert!(path.join("main.luau").exists());
    }

    #[test]
    fn test_edit_custom_template_files() {
        let temp = tempdir().unwrap();

        let old_name = "old".to_string();
        let new_name = "new".to_string();

        Template::create_custom_template_files(&temp.path().to_path_buf(), &old_name, false).unwrap();

        let result = Template::edit_custom_template_files(&temp.path().to_path_buf(), old_name, new_name.clone());
        assert!(result.is_ok());
        let new_path = result.unwrap();

        assert_eq!(new_path.file_name().unwrap().to_str().unwrap(), new_name.as_str())
    }

    #[test]
    fn test_delete_custom_template_files() {
        let temp = tempdir().unwrap();

        let name = "template".to_string();

        Template::create_custom_template_files(&temp.path().to_path_buf(), &name, false).unwrap();

        assert!(Template::delete_custom_template_files(&temp.path().to_path_buf(), name).is_ok())
    }
}