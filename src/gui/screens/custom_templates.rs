use imgui::Ui;
use crate::gui::ui::ScreenState;
use crate::gui::fonts::Fonts;
use crate::managers::template::{get_custom_templates, Template};
use crate::utils::dialogs::error_dialog;
use crate::utils::ui_helpers::{create_imgui_window, main_menu_style, text_center_spacing};

pub fn custom_templates_window(ui: &mut Ui, screen_state: &mut ScreenState, existing_templates: &mut Vec<Template>, fonts: &Fonts) {
    create_imgui_window(ui, "Custom Templates")
        .build(|| {
            let title_text: &str = "Custom Templates";
            let push = ui.push_font(fonts.title_font);
            ui.same_line_with_pos(text_center_spacing(title_text));
            ui.text(title_text);
            push.end();

            let new_text: &str = "New Template";
            let edit_text: &str = "Edit Templates";
            let back_text: &str = "Main Menu";

            for _ in 1..6 {
                ui.new_line();
            }

            main_menu_style(ui, || {
                let push = ui.push_font(fonts.big_font);
                ui.same_line_with_pos(text_center_spacing(new_text));
                if ui.button(new_text) {
                    let result = get_custom_templates();
                    if let Err(e) = result {
                        error_dialog("Get Templates Failure", "Failed to get custom templates", &e);
                    } else {
                        *existing_templates = result
                            .unwrap()
                            .into_iter()
                            .map(|name| Template::Custom(name))
                            .collect();
                        *screen_state = ScreenState::NewTemplate;
                    }
                    
                }
                ui.new_line();
                ui.same_line_with_pos(text_center_spacing(edit_text));
                if ui.button(edit_text) {
                    let result = get_custom_templates();
                    if let Err(e) = result {
                        error_dialog("Get Templates Failure", "Failed to get custom templates", &e);
                    } else {
                        *existing_templates = result
                            .unwrap()
                            .into_iter()
                            .map(|name| Template::Custom(name))
                            .collect();
                        *screen_state = ScreenState::NewTemplate;
                    }

                    *screen_state = ScreenState::EditTemplates;
                }
                ui.new_line();
                ui.same_line_with_pos(text_center_spacing(back_text));
                if ui.button(back_text) {
                    *screen_state = ScreenState::MainMenu;
                }
                push.end();
            });
        });
}