use std::fs;
use std::io::{Error, ErrorKind};
use imgui::Ui;
use opener::OpenError;
use crate::gui::ScreenState;
use crate::gui::fonts::Fonts;
use crate::managers::template::get_custom_templates_dir;
use crate::utils::dialogs::error_dialog;
use crate::utils::ui_helpers::create_imgui_window;

pub fn custom_templates_window(ui: &mut Ui, screen_state: &mut ScreenState, fonts: &Fonts) {
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
                        error_dialog("Error", "Failed to create templates path", &e);
                        return;
                    }
                }

                let result = opener::open(&templates_path);
                if let Err(e) = result {
                    let e = match e {
                        OpenError::Io(e) => {
                            e
                        }
                        OpenError::Spawn {cmds, source } => {
                            Error::new(source.kind(), format!("cmds: {}, Err: {}", cmds, source))
                        }
                        OpenError::ExitStatus {cmd, status, stderr} => {
                            Error::new(ErrorKind::Other, format!("cmd: {}, status: {}, stderr: {}", cmd, status, stderr))
                        }
                        _ => {
                            Error::new(ErrorKind::Other, "panic!")
                        }
                    };
                    error_dialog("Error", "Failed to open templates path", &e);
                    return;
                }
            }
            ui.same_line();
            if ui.button("back") {
                *screen_state = ScreenState::MainMenu;
            }
        });
}