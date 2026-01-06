use std::fs::{create_dir, create_dir_all, read_dir};
use std::{fs, io};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use include_dir::include_dir;
use crate::gui::state::CreateTemplateData;

#[derive(Clone)]
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
    
    pub fn create_custom_template(create_data: &CreateTemplateData, existing_templates: &Vec<Template>) -> io::Result<(PathBuf, Option<String>)> {
        if let Err(e) = create_data.has_sufficient_info(existing_templates) {
            return Err(e)
        }

        let templates_dir = get_custom_templates_dir();

        if !templates_dir.exists() {
            create_dir_all(&templates_dir)?;
        }

        let new_template_path = templates_dir.join(&create_data.template_name);
        create_dir(&new_template_path)?;

        let mut warning = None;
        if create_data.create_main {
            if let Err(e) = fs::write(new_template_path.join("main.luau"), "") {
                warning = Some(format!("Failed to create main.luau.\nContinuing anyways.\nError: {}", e))
            }
        }
        
        Ok((new_template_path, warning))
    }
}

#[derive(Copy, Clone)]
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

pub fn get_custom_templates_dir() -> PathBuf {
    dirs::document_dir().unwrap().join("DriftScriptManager").join("templates")
}

pub fn get_custom_templates() -> io::Result<Vec<String>> {
    let templates_dir = get_custom_templates_dir();

    if !templates_dir.exists() {
        create_dir_all(&templates_dir)?;
        return Ok(Vec::new());
    }

    let mut templates = Vec::new();

    for entry in read_dir(templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    templates.push(name_str.to_string());
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

pub fn copy_embedded_template(template: &EmbeddedTemplate, script_path: &Path) -> std::io::Result<()> {
    let template_dir = TEMPLATES_DIR
        .get_dir(template.name())
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Template not found"))?;

    copy_embedded_dir_recursive(&template_dir, &script_path)?;

    Ok(())
}


fn copy_embedded_dir_recursive(dir: &include_dir::Dir, dst: &Path) -> std::io::Result<()> {
    for file in dir.files() {
        let file_path = dst.join(file.path().file_name().unwrap());

        std::fs::write(&file_path, file.contents())?;
    }


    for dir in dir.dirs() {
        let dir_path = dst.join(dir.path().file_name().unwrap());

        create_dir_all(&dir_path)?;

        copy_embedded_dir_recursive(dir, &dir_path)?;
    }

    Ok(())
}

fn copy_custom_template(template_name: &str, script_path: &Path) -> io::Result<()> {
    let template_dir = get_custom_templates_dir().join(template_name);

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
            std::fs::copy(&path, &dst_path)?;
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
}