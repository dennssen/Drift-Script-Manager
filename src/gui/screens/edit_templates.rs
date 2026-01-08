use imgui::sys::ImVec2;
use imgui::Ui;
use crate::utils::icons;
use crate::gui::state::EditTemplateData;
use crate::gui::ui::ScreenState;
use crate::managers::template::Template;
use crate::utils::ui_helpers::{create_imgui_window, list_box, selectable, separator_text};

pub fn edit_templates_screen(ui: &mut Ui, screen_state: &mut ScreenState, edit_template_data: &mut EditTemplateData, existing_templates: &Vec<Template>) {
    create_imgui_window(ui, "Edit Templates")
        .build(|| {
            if ui.button("Open Template\nDirectory") {
                // Open Templates Directory
            }

            let box_width = ui.content_region_avail()[0] * 0.66;
            let box_height = ui.content_region_max()[1] * 0.19;

            list_box("Templates", ImVec2::new(box_width, box_height), || {
                for template in existing_templates {
                    let name = template.name();
                    let selected = name == edit_template_data.selected_template;
                    selectable(name.as_str(), selected, ImVec2::new(box_width, box_height / 6.0), || {
                        if selected {
                            return;
                        }

                        edit_template_data.selected_template = name.clone();
                        edit_template_data.template_name = name.clone();
                    });

                    /* Doesn't show. Fix or rework
                    if selected {
                        ui.same_line();
                        if ui.button(icons::TRASH) {
                            // Delete Template
                        }
                    }*/
                }
            });

            ui.new_line();
            if !edit_template_data.selected_template.is_empty() {
                if ui.button("Open Directory") {
                    // Open Template Directory
                }

                // Not like expected
                separator_text(ui, edit_template_data.selected_template.as_str());
                ui.indent();
                ui.input_text("Template Name", &mut edit_template_data.template_name).build();
                ui.new_line();
                if ui.button("Save") {
                    // Save changes
                }
                ui.same_line();
                if ui.button("Cancel") {
                    // Deselect template
                }
                ui.unindent();
            }
            ui.new_line();
            if ui.button("Exit") {
                *screen_state = ScreenState::CustomTemplates;
            }
        });
}