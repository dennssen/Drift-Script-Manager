pub mod state;
mod screens;
pub mod fonts;
pub mod setup;

use state::{BuildData, ScreenData, ProjectsData, CreateData};
use fonts::Fonts;
use screens::project_info::project_info_screen;
use screens::save_project::save_project_edit;
use screens::build_project::build_project_screen;
use screens::create_project::create_project_screen;
use crate::utils;
use utils::ui_helpers::{create_imgui_window};
use utils::dialogs::error_dialog;
use imgui::sys::{igGetMainViewport, igSetNextWindowPos, igSetNextWindowSize, ImGuiCond, ImGuiViewport, ImVec2};
use imgui::Ui;
use crate::gui::screens::custom_templates::custom_templates_window;
use crate::gui::screens::main_menu::main_menu_screen;

pub struct GuiInfo {
    screen_state: ScreenState,
    screen_data: ScreenData,
    projects: ProjectsData
}

pub enum ScreenState {
    MainMenu,
    SetProjectInfo,
    CreateProject,
    CreatingProject,
    SetBuildInfo,
    BuildingProject,
    EditProjectInfo,
    SavingProjectInfo,
    TemplateInfo,
}

impl GuiInfo {
    pub fn new() -> Self {
        Self {
            screen_state: ScreenState::MainMenu,
            screen_data: ScreenData::new(),
            projects: ProjectsData::new()
        }
    }
}

fn set_gui_window(){
    unsafe {
        let viewport: *mut ImGuiViewport = igGetMainViewport();
        igSetNextWindowPos((*viewport).WorkPos, ImGuiCond::default(), ImVec2::zero());
        igSetNextWindowSize((*viewport).Size, ImGuiCond::default());
    }
}

pub fn render_ui(ui: &mut Ui, gui_info: &mut GuiInfo, fonts: &Fonts) {
    set_gui_window();

    match gui_info.screen_state {
        ScreenState::MainMenu => {
            main_menu_screen(ui, &mut gui_info.screen_state, &mut gui_info.projects.build_project, &mut gui_info.projects.edit_project, fonts);
        }
        ScreenState::SetProjectInfo => {
            project_info_screen(ui, &mut gui_info.screen_state, &mut gui_info.projects.new_project_info, true, fonts);
        }
        ScreenState::EditProjectInfo => {
            if gui_info.projects.edit_project.is_none() {
                // Should be physically impossible to get here
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            }

            project_info_screen(ui, &mut gui_info.screen_state, gui_info.projects.edit_project.as_mut().unwrap(), false, fonts);
        }
        ScreenState::SetBuildInfo => {
            if gui_info.projects.build_project.is_none() {
                // Should be physically impossible to get here
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            }

            build_project_screen(ui, &mut gui_info.screen_state, &mut gui_info.screen_data.build_data, gui_info.projects.build_project.as_mut().unwrap(), fonts);
        }
        ScreenState::CreateProject => {
            if let Err(e) = create_project_screen(ui, &mut gui_info.screen_state, &mut gui_info.screen_data.create_data, &mut gui_info.projects.new_project_info, fonts) {
                error_dialog("Error", e.to_string().as_str());
                gui_info.screen_state = ScreenState::MainMenu;
            }
        }
        ScreenState::CreatingProject => {
            create_imgui_window(ui, "Creating...")
                .build(|| {
                    let push = ui.push_font(fonts.header_font);
                    ui.text("Creating Project");
                    push.end();
                });

            let create_result = gui_info.projects.new_project_info.create_project_files(&mut gui_info.screen_data.create_data);
            if let Err(err) = create_result {
                error_dialog("Creation Failure", err.as_str());
            }

            gui_info.projects.new_project_info.reset_project_data();
            gui_info.screen_data.create_data = CreateData::default();
            gui_info.screen_state = ScreenState::MainMenu;
        }
        ScreenState::SavingProjectInfo => {
            create_imgui_window(ui, "Saving...")
                .build(|| {
                    let push = ui.push_font(fonts.header_font);
                    ui.text("Saving Project");
                    push.end();
                });

            gui_info.projects.edit_project.is_none().then(|| {
                error_dialog("Missing Project", "Project cannot be edited because it does not exist");
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            });

            save_project_edit(&mut gui_info.screen_state, gui_info.projects.edit_project.as_ref().unwrap());
        }
        ScreenState::BuildingProject => {
            create_imgui_window(ui, "Building...")
                .build(|| {
                    let push = ui.push_font(fonts.header_font);
                    ui.text("Building Project");
                    push.end();
                });

            gui_info.projects.build_project.is_none().then(|| {
                error_dialog("Missing Project", "Project cannot be built because it does not exist");
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            });

            gui_info.projects.build_project.as_ref().unwrap().build(&gui_info.screen_data.build_data);
            gui_info.screen_data.build_data = BuildData::default();
            gui_info.screen_state = ScreenState::MainMenu;
        }
        ScreenState::TemplateInfo => {
            custom_templates_window(ui, &mut gui_info.screen_state, fonts)
        }
    }

}