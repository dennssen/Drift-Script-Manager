use imgui::{ItemHoveredFlags, Ui};
use std::sync::{Mutex};

use crate::utils::ui_helpers::{create_imgui_window, keyword_list_box};
use crate::gui::fonts::Fonts;
use crate::project::drift_project::{DriftProject};
use crate::gui::ui::ScreenState;
use crate::managers::data::get_app_data;
use crate::managers::template::{get_custom_templates, Template};
use crate::utils::dialogs::{error_dialog, warn_dialog};

static SEARCH_FILTER: Mutex<String> = Mutex::new(String::new());

pub enum ProjectMode {
    New,
    Edit
}

pub fn project_info_screen(
    ui: &mut Ui,
    screen_state: &mut ScreenState,
    drift_project: &mut DriftProject,
    project_mode: ProjectMode,
    custom_templates: &mut Vec<Template>,
    fonts: &Fonts
) {
    let (window_name, header) = match project_mode {
        ProjectMode::New => {
            ("Creating Project...", "New Project")
        }
        ProjectMode::Edit => {
            ("Editing Project...", "Edit Project")
        }
    };

    create_imgui_window(&ui, window_name)
        .build(|| {
            let push = ui.push_font(fonts.header_font);
            ui.text(header);
            push.end();
            ui.new_line();

            ui.input_text("Author", &mut drift_project.package_info.author).build();
            ui.input_text("Project Name", &mut drift_project.package_info.project_name).build();

            let mut suggested_script_name: String = String::new();

            if !&drift_project.package_info.author.is_empty() && !&drift_project.package_info.project_name.is_empty() {
                suggested_script_name = drift_project.package_info.author.to_lowercase().replace(" ", "") + "." + drift_project.package_info.project_name.to_lowercase().replace(" ", "").as_str();
            }

            ui.input_text("Script Name", &mut drift_project.package_info.script_name).hint(suggested_script_name.clone()).build();
            ui.input_text("Version", &mut drift_project.package_info.version).chars_decimal(true).build();

            let multiline_width: f32 = ui.content_region_avail()[0] * 0.66;
            let multiline_height: f32 = ui.content_region_max()[1] * 0.19;
            ui.input_text_multiline("Description", &mut drift_project.package_info.description, [multiline_width, multiline_height]).build();


            let mut search = SEARCH_FILTER.lock().unwrap();

            let mut app_data = get_app_data().lock().unwrap();

            keyword_list_box(ui, &mut search, &app_data.keywords.clone(), &mut drift_project.package_info.keywords, "##KeywordsList", multiline_width, multiline_height, |new_keyword| {
                app_data.keywords.push(new_keyword);
            });

            drop(search);

            ui.new_line();
            ui.new_line();
            if ui.button("Back") {
                *screen_state = ScreenState::MainMenu;
            }
            ui.same_line();

            match project_mode {
                ProjectMode::New => {
                    let sufficient = drift_project.has_sufficient_info();
                    let is_disabled: bool = sufficient.is_err();

                    let disabled = ui.begin_disabled(is_disabled);
                    if ui.button("Next") {
                        if drift_project.package_info.script_name.is_empty() {
                            drift_project.package_info.script_name = suggested_script_name;
                        }

                        drift_project.directory_name = drift_project.package_info.project_name.replace(" ", "-");

                        let result = get_custom_templates();

                        match result {
                            Err(e) => {
                                warn_dialog("Get Templates Failure", &format!("Failed to get custom templates.\nError: {}", e));
                            }
                            Ok(templates) => {
                                *custom_templates = templates;
                            }
                        }
                        *screen_state = ScreenState::CreateProject;
                    }
                    disabled.end();
                    if ui.is_item_hovered_with_flags(ItemHoveredFlags::ALLOW_WHEN_DISABLED) && is_disabled {
                        ui.tooltip_text(sufficient.err().unwrap());
                    }
                }
                ProjectMode::Edit => {
                    if ui.button("Done") {
                        *screen_state = ScreenState::SavingProjectInfo;
                    }
                }
            }
        });
}

