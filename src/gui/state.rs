use std::io;
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

pub struct EditTemplateData {
    pub template_name: String,
    pub selected_template: Option<Template>,
    pub sufficient_result: Option<io::Result<()>>,
}

impl Default for EditTemplateData {
    fn default() -> Self {
        Self {
            template_name: String::default(),
            selected_template: None,
            sufficient_result: None,
        }
    }
}

pub struct CreateTemplateData {
    pub template_name: String,
    pub create_main: bool,
    pub sufficient_result: Option<io::Result<()>>,
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
