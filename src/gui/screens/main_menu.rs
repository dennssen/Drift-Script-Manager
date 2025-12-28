use std::io::Result;
use imgui::{StyleVar, Ui};
use crate::gui::ui::{ScreenState};
use crate::gui::fonts::Fonts;
use crate::managers::data::get_app_data;
use crate::project::drift_project::{DriftProject, ProjectPaths};
use crate::project::package_info::PackageInfo;
use crate::utils::dialogs::warn_dialog;
use crate::utils::ui_helpers::{create_imgui_window, text_center_spacing};

pub fn main_menu_screen(ui: &mut Ui, screen_state: &mut ScreenState, build_project: &mut Option<DriftProject>, edit_project: &mut Option<DriftProject>, fonts: &Fonts) {
    create_imgui_window(ui, "Main Menu")
        .build(|| {
            let title_text: &str = "Drift Script Manager";
            let push = ui.push_font(fonts.title_font);
            ui.same_line_with_pos(text_center_spacing(title_text));
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
            ui.same_line_with_pos(text_center_spacing(new_text));
            if ui.button(new_text) {
                *screen_state = ScreenState::SetProjectInfo;
            }
            ui.new_line();
            ui.same_line_with_pos(text_center_spacing(edit_text));
            if ui.button(edit_text) {
                set_edit_screen(screen_state, edit_project);
            }
            ui.new_line();
            ui.same_line_with_pos(text_center_spacing(build_text));
            if ui.button(build_text) {
                set_build_screen(screen_state, build_project);
            }
            ui.new_line();
            push.end();
            let push = ui.push_font(fonts.medium_font);
            ui.same_line_with_pos(text_center_spacing(template_text));
            if ui.button(template_text) {
                *screen_state = ScreenState::TemplateInfo;
            }
            ui.new_line();
            push.end();
            style_push1.end();
            style_push2.end();
            style_push3.end();
        });
}

fn set_build_screen(screen_state: &mut ScreenState, build_project: &mut Option<DriftProject>) {
    *screen_state = ScreenState::SetBuildInfo;

    let drift_project: Result<DriftProject> = try_get_project();

    if drift_project.is_err() {
        warn_dialog("File Dialog Failure", drift_project.unwrap_err().to_string().as_str());

        *screen_state = ScreenState::MainMenu;
        return;
    }

    *build_project = Some(drift_project.unwrap());
}

fn set_edit_screen(screen_state: &mut ScreenState, edit_project: &mut Option<DriftProject>) {
    *screen_state = ScreenState::EditProjectInfo;

    let drift_project: Result<DriftProject> = try_get_project();

    if drift_project.is_err() {
        warn_dialog("File Dialog Failure", drift_project.unwrap_err().to_string().as_str());

        *screen_state = ScreenState::MainMenu;
        return;
    }

    *edit_project = Some(drift_project.unwrap());
}

fn try_get_project() -> Result<DriftProject> {
    let (package_info, package_path) = PackageInfo::get_package_file()?;

    let project_paths: ProjectPaths = ProjectPaths::validate_project_structure(package_path, &package_info)?;

    let project: DriftProject = DriftProject::project_from_package(package_info, project_paths);

    let mut app_data = get_app_data().lock().unwrap();
    app_data.update_keywords(&project.package_info.keywords);
    Ok(project)
}