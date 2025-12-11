use crate::drift_project::DriftProject;
use crate::gui::{ScreenState};

pub fn save_project_edit(screen_state: &mut ScreenState, project: &DriftProject) {
    if let Err(_) = project.save() {
        // log
    }
    *screen_state = ScreenState::MainMenu;
}