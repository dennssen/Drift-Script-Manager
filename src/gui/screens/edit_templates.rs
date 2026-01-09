use std::io;
use std::sync::Mutex;
use imgui::sys::ImVec2;
use imgui::{ItemHoveredFlags, Ui};
use crate::utils::icons;
use crate::gui::state::EditTemplateData;
use crate::gui::ui::ScreenState;
use crate::managers::template::{get_custom_templates_dir, Template};
use crate::managers::template::Template::Custom;
use crate::utils::dialogs::error_dialog;
use crate::utils::error_helper::open_error_to_io;
use crate::utils::ui_helpers::{create_imgui_window, list_box, selectable, separator_text};

static DELETE_CONFIRMATION: Mutex<bool> = Mutex::new(false);

pub fn edit_templates_screen(ui: &mut Ui, screen_state: &mut ScreenState, edit_template_data: &mut EditTemplateData, existing_templates: &mut Vec<Template>) {
    create_imgui_window(ui, "Edit Templates")
        .build(|| {
            if ui.button(format!("{} Open Templates Directory", icons::FOLDER)) {
                let dir = get_custom_templates_dir();
                let result = opener::open(&dir);
                if let Err(e) = result {
                    error_dialog(
                        "Open Error",
                        &format!("Failed to open templates directory.\nDirectory can be found here:\n{}", dir.to_str().unwrap_or_default()),
                        &open_error_to_io(&e)
                    )
                }
            }

            ui.new_line();

            let box_width = ui.content_region_avail()[0] * 0.66;
            let box_height = ui.content_region_max()[1] * 0.19;

            list_box("Templates", ImVec2::new(box_width, box_height), || {
                for template in existing_templates.iter() {
                    let name = template.name();
                    let mut selected: bool = false;
                    if let Some(template) = edit_template_data.selected_template.as_ref() {
                        selected = template.name() == name;
                    }
                    selectable(name.as_str(), selected, ImVec2::new(box_width, box_height / 6.0), || {
                        if selected {
                            return;
                        }

                        *DELETE_CONFIRMATION.lock().unwrap() = false;
                        edit_template_data.selected_template = Some(template.clone());
                        edit_template_data.template_name = name.clone();
                    });
                }
            });

            ui.new_line();
            if let Some(selected_template) = edit_template_data.selected_template.as_ref() {
                separator_text(ui, selected_template.name());

                ui.indent();
                if ui.button(format!("{} Open Directory", icons::FOLDER)) {
                    let dir = get_custom_templates_dir().join(&selected_template.name());
                    let result = opener::open(&dir);
                    if let Err(e) = result {
                        error_dialog(
                            "Open Error",
                            &format!("Failed to open templates directory.\nDirectory can be found here:\n{}", dir.to_str().unwrap_or_default()),
                            &open_error_to_io(&e)
                        )
                    }
                }
                ui.new_line();
                ui.text("Info");
                let input_edited = ui.input_text("Template Name", &mut edit_template_data.template_name).build();

                if input_edited || edit_template_data.sufficient_result.is_none() {
                    edit_template_data.sufficient_result = Some(Template::has_sufficient_info(&edit_template_data.template_name, existing_templates));
                }

                ui.new_line();

                if selected_template_action_buttons(
                    ui,
                    selected_template,
                    &edit_template_data.template_name,
                    existing_templates,
                    edit_template_data.sufficient_result.as_ref().unwrap(),
                ) {
                    edit_template_data.selected_template = None;
                }

                ui.unindent();
                ui.separator();
            }
            ui.new_line();
            if ui.button("Exit") {
                *screen_state = ScreenState::CustomTemplates;
            }
        });
}

fn selected_template_action_buttons(
    ui: &Ui,
    selected_template: &Template,
    new_template_name: &String,
    existing_templates: &mut Vec<Template>,
    sufficient_result: &io::Result<()>,
) -> bool {
    if !*DELETE_CONFIRMATION.lock().unwrap() {
        let disabled = sufficient_result.is_err();
        let push = ui.begin_disabled(disabled);
        if ui.button("Save") {
            let result = selected_template.edit_custom_template(Custom(new_template_name.clone()));
            match result {
                Err(e) => {
                    error_dialog("Save Failure", "Failed to edit custom template", &e);
                }
                Ok(new_templates_list) => {
                    // Possibly info dialog

                    *existing_templates = new_templates_list;
                    return true
                }
            }
        }
        push.end();

        if ui.is_item_hovered_with_flags(ItemHoveredFlags::ALLOW_WHEN_DISABLED) && disabled {
            if let Err(e) = sufficient_result {
                ui.tooltip_text(e.to_string());
            }
        }
        ui.same_line();
        if ui.button("Cancel") {
            return true
        }
        ui.same_line();
        if ui.button(format!("{} Delete", icons::TRASH)) {
            *DELETE_CONFIRMATION.lock().unwrap() = true;
        }

    } else {
        ui.text(format!("Are you sure you want to delete {}?", selected_template.name()));
        if ui.button("Confirm") {
            let result = selected_template.delete_custom_template();
            match result {
                Err(e) => {
                    error_dialog("Delete Failure", "Failed to delete template", &e);
                }
                Ok(new_templates_list) => {
                    *existing_templates = new_templates_list;

                    *DELETE_CONFIRMATION.lock().unwrap() = false;
                    return true
                }
            }
        }
        ui.same_line();
        if ui.button("CANCEL") {
            *DELETE_CONFIRMATION.lock().unwrap() = false;
        }
    }

    false
}