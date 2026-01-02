use std::io;
use std::io::{Error, ErrorKind};
use crate::project::drift_project::DriftProject;
use crate::managers::template::{Template, EmbeddedTemplate};

pub struct ProjectsData {
    pub new_project_info: DriftProject,
    pub edit_project: Option<DriftProject>,
    pub build_project: Option<DriftProject>,
}

impl ProjectsData {
    pub fn new() -> Self {
        Self {
            new_project_info: DriftProject::new(),
            edit_project: None,
            build_project: None,
        }
    }
}

pub struct CreateProjectData {
    pub open_directory: bool,
    pub create_repo: bool,
    pub template: Template,
}

impl CreateProjectData {
    pub fn default() -> Self {
        Self {
            open_directory: false,
            create_repo: false,
            template: Template::Embedded(EmbeddedTemplate::Empty),
        }
    }
}

pub struct BuildProjectData {
    pub open_directory: bool,
    pub version_tag: bool
}

impl BuildProjectData {
    pub fn default() -> Self {
        Self {
            open_directory: false,
            version_tag: false,
        }
    }
}

pub struct ScreenData {
    pub create_data: CreateProjectData,
    pub build_data: BuildProjectData
}

impl ScreenData {
    pub fn new() -> Self {
        Self {
            create_data: CreateProjectData::default(),
            build_data: BuildProjectData::default()
        }
    }
}

pub struct CreateTemplateData {
    pub template_name: String,
    pub create_main: bool,
    pub sufficient_result: Option<io::Result<()>>,
}

impl CreateTemplateData {
    pub fn has_sufficient_info(&self, existing_templates: &Vec<Template>) -> io::Result<()> {
        if let Err(e) = self.has_valid_name() {
            return Err(e)
        }
        
        if let Err(e) = self.is_unique(existing_templates) {
            return Err(e)
        }
        
        Ok(())
    }
    
    fn is_unique(&self, existing_templates: &Vec<Template>) -> io::Result<()> {
        for template in existing_templates {
            let name = template.name();
            if name == self.template_name {
                return Err(Error::new(ErrorKind::AlreadyExists, "A template with this name already exists"));
            }
        }
        
        Ok(())
    }
    
    fn has_valid_name(&self) -> io::Result<()> {
        let name = &self.template_name;
        
        if name.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Template name cannot be empty"));
        }
        
        if name.replace(" ", "").is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Template name cannot consist of whitespace only"));
        }
        
        Ok(())
    }
}

impl Default for CreateTemplateData {
    fn default() -> Self {
         Self {
            template_name: String::default(),
            create_main: bool::default(),
            sufficient_result: None
        }
    }
}
