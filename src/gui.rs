pub mod project_info;
pub mod build_project;
pub mod create_project;
mod save_project;

use std::fs;
use crate::drift_project::{DriftProject, PackageInfo};
use crate::fonts::Fonts;
use crate::gui::project_info::{project_info_screen};
use crate::gui::save_project::save_project_edit;
use crate::util::{create_imgui_window, error_dialog, text_center_spacing};
use crate::template_manager::{get_custom_templates_dir, EmbeddedTemplate, Template};
use imgui::sys::{igGetMainViewport, igSetNextWindowPos, igSetNextWindowSize, ImGuiCond, ImGuiViewport, ImVec2};
use imgui::{StyleVar, Ui};
use crate::gui::build_project::build_project_screen;
use crate::gui::create_project::create_project_screen;

struct ProjectsData {
    new_project_info: DriftProject,
    edit_project: Option<DriftProject>,
    build_project: Option<DriftProject>,
}

impl ProjectsData {
    fn new() -> Self {
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
    fn default() -> Self {
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
    fn default() -> Self {
        Self {
            open_directory: false,
            version_tag: false,
        }
    }
}

struct ScreenData {
    create_data: CreateData,
    build_data: BuildData
}

impl ScreenData {
    fn new() -> Self {
        Self {
            create_data: CreateData::default(),
            build_data: BuildData::default()
        }
    }
}

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
            create_imgui_window(ui, "Main Menu")
                .build(|| {
                    let title_text: &str = "Drift Script Manager";
                    let push = ui.push_font(fonts.title_font);
                    unsafe {
                        ui.same_line_with_pos(text_center_spacing(title_text));
                    }
                    ui.text(title_text);
                    push.end();

                    for _ in 1..6 {
                        ui.new_line();
                    }

                    let new_text: &str = "New Project";
                    let build_text: &str = "Build Project";
                    let edit_text: &str = "Edit Project";
                    let template_text: &str = "Create Template";

                    let style_push1 = ui.push_style_var(StyleVar::ItemSpacing([8.0, 10.0]));
                    let style_push2 = ui.push_style_var(StyleVar::FrameRounding(10.0));
                    let style_push3 = ui.push_style_var(StyleVar::FramePadding([10.0, 12.0]));
                    let push = ui.push_font(fonts.big_font);
                    unsafe {
                        ui.same_line_with_pos(text_center_spacing(new_text));
                    }
                    if ui.button(new_text) {
                        gui_info.screen_state = ScreenState::SetProjectInfo;
                    }
                    ui.new_line();
                    unsafe {
                        ui.same_line_with_pos(text_center_spacing(edit_text));
                    }
                    if ui.button(edit_text) {
                        set_edit_screen(gui_info);
                    }
                    ui.new_line();
                    unsafe {
                        ui.same_line_with_pos(text_center_spacing(build_text));
                    }
                    if ui.button(build_text) {
                        set_build_screen(gui_info);
                    }
                    ui.new_line();
                    push.end();
                    let push = ui.push_font(fonts.medium_font);
                    unsafe {
                        ui.same_line_with_pos(text_center_spacing(template_text));
                    }
                    if ui.button(template_text) {
                        gui_info.screen_state = ScreenState::TemplateInfo;
                    }
                    ui.new_line();
                    push.end();
                    style_push1.end();
                    style_push2.end();
                    style_push3.end();
                });
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
            create_imgui_window(ui, "Custom Templates")
                .build(|| {
                    let header_text: &str = "Templates Info";
                    let push = ui.push_font(fonts.header_font);
                    ui.text(header_text);
                    push.end();

                    ui.new_line();

                    ui.text("Custom Templates allow you to create your own reusable project structures. \
                    \nWhen you click the 'Open Folder' button, a folder will open where you can \
                    \ncreate new template directories. \
                    \n\nTo create a custom template, simply create a new folder \
                    \nwith your template name and add any files and folders you'd like to include in your projects. \
                    \nIf you include a main.luau file, it will be automatically linked to your project - otherwise, \
                    \nit's optional. You can organize your files however you like with as many subfolders as needed. \
                    \nOnce you've set up your template, it will automatically appear in the template selector \
                    \nthe next time you create a project.");

                    ui.new_line();

                    if ui.button("Open folder") {
                        let templates_path = get_custom_templates_dir();
                        if !templates_path.exists() {
                            let result = fs::create_dir_all(&templates_path);
                            if let Err(e) = result {
                                error_dialog("Error", e.to_string().as_str());
                                return;
                            }
                        }

                        let result = opener::open(&templates_path);
                        if let Err(e) = result {
                            error_dialog("Error", e.to_string().as_str());
                            return;
                        }
                    }
                    ui.same_line();
                    if ui.button("back") {
                        gui_info.screen_state = ScreenState::MainMenu;
                    }
                });
        }
    }

}

fn set_build_screen(gui_info: &mut GuiInfo) {
    gui_info.screen_state = ScreenState::SetBuildInfo;
    let try_package = PackageInfo::get_package_file();

    if try_package.is_err() {
        gui_info.screen_state = ScreenState::MainMenu;
        return;
    }

    let (package_info, package_path) = try_package.unwrap();

    let drift_project: Option<DriftProject> = DriftProject::project_from_package(package_info, package_path);

    drift_project.is_none().then(|| {
        error_dialog("Parse Failure", "Failed to parse project from package.json.\nCheck your file structure.");

        gui_info.screen_state = ScreenState::MainMenu;
        return;
    });

    gui_info.projects.build_project = drift_project;
}

fn set_edit_screen(gui_info: &mut GuiInfo) {
    gui_info.screen_state = ScreenState::EditProjectInfo;
    let try_package = PackageInfo::get_package_file();

    if try_package.is_err() {
        gui_info.screen_state = ScreenState::MainMenu;
        return;
    }

    let (package_info, package_path) = try_package.unwrap();

    let drift_project: Option<DriftProject> = DriftProject::project_from_package(package_info, package_path);

    drift_project.is_none().then(|| {
        error_dialog("Parse Failure", "Failed to parse project from package.json.\nCheck your file structure.");

        gui_info.screen_state = ScreenState::MainMenu;
        return;
    });

    gui_info.projects.edit_project = drift_project;
}