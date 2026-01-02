use std::io;
use imgui::{ItemHoveredFlags, Ui};
use crate::project::drift_project::DriftProject;
use crate::gui::fonts::Fonts;
use crate::managers;
use managers::git::has_git;
use managers::template::{EmbeddedTemplate, Template};
use crate::gui::ui::{CreateProjectData, ScreenState};
use crate::utils::ui_helpers::{create_imgui_window, directory_input};

pub fn create_project_screen(
    ui: &mut Ui,
    screen_state: &mut ScreenState,
    create_data: &mut CreateProjectData,
    project: &mut DriftProject,
    custom_templates: &Vec<Template>,
    fonts: &Fonts
) -> io::Result<()> {
    create_imgui_window(ui, "Creating Project...")
        .build(|| {
            let push = ui.push_font(fonts.header_font);
            ui.text("New Project");
            push.end();

            ui.new_line();
            ui.new_line();
            ui.input_text("Directory Name", &mut project.directory_name)
                .chars_noblank(true)
                .build();
            directory_input(ui, "Project Location", &mut project.project_location);

            let embedded_template_names: Vec<Template> = EmbeddedTemplate::all()
                .iter()
                .map(|t| Template::Embedded(*t))
                .collect();

            if let Some(_) = ui.begin_combo("Template", &create_data.template.name()) {
                for template in embedded_template_names.iter().chain(custom_templates.iter()) {
                    let selected: bool = create_data.template.name() == template.name();
                    if selected {
                        ui.set_item_default_focus();
                    }

                    let clicked = ui.selectable_config(template.name())
                        .selected(selected)
                        .build();

                    if clicked {
                        create_data.template = template.clone();
                    }
                }
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Choose a template for your project to start with.\nLearn to make your own through the Main Menu!");
            }
            ui.disabled(!has_git(), || {
               ui.checkbox("Create git repo", &mut create_data.create_repo); 
            });
            ui.checkbox("Open Project Directory", &mut create_data.open_directory);

            ui.new_line();
            ui.new_line();
            if ui.button("Back") {
                *screen_state = ScreenState::SetProjectInfo;
            }
            ui.same_line();
            let creatable = project.is_creatable();
            let is_disabled = creatable.is_err();
            ui.disabled(is_disabled, || {
                if ui.button("Create") {
                    *screen_state = ScreenState::CreatingProject;
                }

                if ui.is_item_hovered_with_flags(ItemHoveredFlags::ALLOW_WHEN_DISABLED) && is_disabled {
                    ui.tooltip_text(creatable.err().unwrap());
                }
            });
        });

    Ok(())
}