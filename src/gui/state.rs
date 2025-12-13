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

pub struct CreateData {
    pub open_directory: bool,
    pub create_repo: bool,
    pub template: Template,
}

impl CreateData {
    pub fn default() -> Self {
        Self {
            open_directory: false,
            create_repo: false,
            template: Template::Embedded(EmbeddedTemplate::Empty),
        }
    }
}

pub struct BuildData {
    pub open_directory: bool,
    pub version_tag: bool
}

impl BuildData {
    pub fn default() -> Self {
        Self {
            open_directory: false,
            version_tag: false,
        }
    }
}

pub struct ScreenData {
    pub create_data: CreateData,
    pub build_data: BuildData
}

impl ScreenData {
    pub fn new() -> Self {
        Self {
            create_data: CreateData::default(),
            build_data: BuildData::default()
        }
    }
}