use crate::project::drift_project::DriftProject;
use crate::gui::ui::{ScreenState};

pub fn save_project_edit(screen_state: &mut ScreenState, project: &DriftProject) {
    if let Err(_) = project.save() {
        // error is pre handled/reported
    }
    *screen_state = ScreenState::MainMenu;
}