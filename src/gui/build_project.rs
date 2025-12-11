use imgui::Ui;
use crate::drift_project::DriftProject;
use crate::fonts::Fonts;
use crate::gui::{BuildData, ScreenState};
use crate::util::{create_imgui_window, directory_input};

pub fn build_project_screen(ui: &Ui, screen_state: &mut ScreenState, build_data: &mut BuildData, project: &mut DriftProject, fonts: &Fonts) {
    create_imgui_window(ui, "Building Project...")
        .build(|| {
            let font = ui.push_font(fonts.header_font);
            ui.text("Build Project");
            font.end();

            ui.new_line();
            ui.new_line();

            directory_input(ui, "Build Output", &mut project.build_path);
            ui.input_text("Version", &mut project.package_info.version).build();


            ui.checkbox("Open Build Directory", &mut build_data.open_directory);
            ui.checkbox("Unique Zip Name", &mut build_data.version_tag);
            if ui.is_item_hovered() {
                ui.tooltip_text("Adds the build version to the zip file name");
            }

            ui.new_line();
            ui.new_line();

            if ui.button("Back") {
                *build_data = BuildData::default();
                *screen_state = ScreenState::MainMenu;
            }
            ui.same_line();
            if ui.button("Build") {
                if let Err(_) = project.save() {
                    *screen_state = ScreenState::MainMenu;
                } else {
                    *screen_state = ScreenState::BuildingProject;
                }
            }
        });
}