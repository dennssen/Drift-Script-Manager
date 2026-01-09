use imgui::{ItemHoveredFlags, Ui};
use crate::gui::state::CreateTemplateData;
use crate::gui::ui::ScreenState;
use crate::managers::template::Template;
use crate::utils::dialogs::{error_dialog, info_dialog, warn_dialog};
use crate::utils::error_helper::open_error_to_io;
use crate::utils::ui_helpers::create_imgui_window;

pub fn new_template_screen(ui: &mut Ui, screen_state: &mut ScreenState, create_template_data: &mut CreateTemplateData, existing_templates: &Vec<Template>) {
    create_imgui_window(ui, "New Template")
        .build(|| {
            ui.new_line();
            ui.new_line();

            let input_edited = ui.input_text("Template Name", &mut create_template_data.template_name).build();

            if input_edited || create_template_data.sufficient_result.is_none() {
                create_template_data.sufficient_result = Some(Template::has_sufficient_info(&create_template_data.template_name, existing_templates));
            }

            ui.checkbox("Create Main.luau", &mut create_template_data.create_main);

            ui.new_line();

            let sufficient_result = create_template_data.sufficient_result.as_ref().unwrap();

            let is_insufficient: bool = sufficient_result.is_err();

            let disabled = ui.begin_disabled(is_insufficient);
            if ui.button("Create") {
                match Template::create_custom_template(create_template_data, existing_templates) {
                    Ok((path, warning)) => {
                        if let Some(warning_msg) = warning {
                            warn_dialog("Template Warning", warning_msg.as_str());
                        }

                        info_dialog("Template Success", "Template created successfully!\nAdd files and folders to the Template and they'll be copied to new projects.");

                        let result = opener::open(&path);
                        if let Err(e) = result {
                            error_dialog(
                                "Template Error",
                                format!("Failed to open Template path.\nTemplate can be found here:\n{}", &path.to_str().unwrap_or_default()).as_str(),
                                &open_error_to_io(&e)
                            );
                        }
                    }
                    Err(e) => {
                        error_dialog("Template Error", "Error while creating template.", &e);
                    }
                }

                *create_template_data = CreateTemplateData::default();
                *screen_state = ScreenState::CustomTemplates;
                return
            }
            disabled.end();
            if ui.is_item_hovered_with_flags(ItemHoveredFlags::ALLOW_WHEN_DISABLED) && is_insufficient {
                if let Err(e) = sufficient_result {
                    ui.tooltip_text(e.to_string());
                }
            }
        });
}