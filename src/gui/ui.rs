pub use crate::gui::state::{BuildProjectData, ScreenData, ProjectsData, CreateProjectData};
use crate::gui::fonts::Fonts;
use crate::gui::screens::project_info::{project_info_screen, ProjectMode};
use crate::gui::screens::save_project::save_project_edit;
use crate::gui::screens::build_project::build_project_screen;
use crate::gui::screens::create_project::create_project_screen;
use crate::gui::screens::custom_templates::custom_templates_window;
use crate::gui::screens::main_menu::main_menu_screen;
use crate::gui::screens::new_template::new_template_screen;
use crate::gui::screens::edit_templates::edit_templates_screen;
use crate::utils;
use utils::ui_helpers::{create_imgui_window};
use utils::dialogs::{error_dialog, warn_dialog};
use imgui::sys::{igGetMainViewport, igSetNextWindowPos, igSetNextWindowSize, ImGuiCond, ImGuiViewport, ImVec2};
use imgui::Ui;
use crate::gui::state::{CreateTemplateData, EditTemplateData};
use crate::managers::template::Template;

pub enum ScreenState {
    MainMenu,
    SetProjectInfo,
    CreateProject,
    CreatingProject,
    SetBuildInfo,
    BuildingProject,
    EditProjectInfo,
    SavingProjectInfo,
    CustomTemplates,
    NewTemplate,
    EditTemplates,
}

pub struct GuiInfo {
    screen_state: ScreenState,
    screen_data: ScreenData,
    projects: ProjectsData,
    custom_templates: Vec<Template>,
    create_template_data: CreateTemplateData,
    edit_template_data: EditTemplateData,
}

impl GuiInfo {
    pub fn new() -> Self {
        Self {
            screen_state: ScreenState::MainMenu,
            screen_data: ScreenData::new(),
            projects: ProjectsData::new(),
            custom_templates: vec![],
            create_template_data: CreateTemplateData::default(),
            edit_template_data: EditTemplateData::default(),
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
            main_menu_screen(
                ui,
                &mut gui_info.screen_state,
                &mut gui_info.projects.build_project,
                &mut gui_info.projects.edit_project,
                fonts
            );
        }
        ScreenState::SetProjectInfo => {
            project_info_screen(
                ui,
                &mut gui_info.screen_state,
                &mut gui_info.projects.new_project_info,
                ProjectMode::New,
                &mut gui_info.custom_templates,
                fonts
            );
        }
        ScreenState::EditProjectInfo => {
            if gui_info.projects.edit_project.is_none() {
                // Should be physically impossible to get here
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            }

            project_info_screen(
                ui,
                &mut gui_info.screen_state,
                gui_info.projects.edit_project.as_mut().unwrap(),
                ProjectMode::Edit,
                &mut gui_info.custom_templates,
                fonts
            );
        }
        ScreenState::SetBuildInfo => {
            if gui_info.projects.build_project.is_none() {
                // Should be physically impossible to get here
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            }

            build_project_screen(
                ui,
                &mut gui_info.screen_state,
                &mut gui_info.screen_data.build_data,
                gui_info.projects.build_project.as_mut().unwrap(),
                fonts
            );
        }
        ScreenState::CreateProject => {
            if let Err(e) = create_project_screen(ui, &mut gui_info.screen_state, &mut gui_info.screen_data.create_data, &mut gui_info.projects.new_project_info, &gui_info.custom_templates, fonts) {
                error_dialog("Error", "Error:", &e);
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
            if let Err(e) = create_result {
                error_dialog("Creation Failure", "Failed to create drift project files", &e);
            }

            gui_info.projects.new_project_info.reset_project_data();
            gui_info.screen_data.create_data = CreateProjectData::default();
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
                warn_dialog("Missing Project", "Project cannot be edited because it does not exist");
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
                warn_dialog("Missing Project", "Project cannot be built because it does not exist");
                gui_info.screen_state = ScreenState::MainMenu;
                return;
            });

            gui_info.projects.build_project.as_ref().unwrap().build(&gui_info.screen_data.build_data);
            gui_info.screen_data.build_data = BuildProjectData::default();
            gui_info.screen_state = ScreenState::MainMenu;
        }
        ScreenState::CustomTemplates => {
            custom_templates_window(ui, &mut gui_info.screen_state, &mut gui_info.custom_templates, fonts)
        }
        ScreenState::NewTemplate => {
            new_template_screen(ui, &mut gui_info.screen_state, &mut gui_info.create_template_data, &gui_info.custom_templates);
        }
        ScreenState::EditTemplates => {
            edit_templates_screen(ui, &mut gui_info.screen_state, &mut gui_info.edit_template_data, &gui_info.custom_templates);
        }
    }

}