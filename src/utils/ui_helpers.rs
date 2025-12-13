use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::null;
use imgui::{FontSource, Ui, Window};
use imgui::sys::{igBeginListBox, igCalcTextSize, igEndListBox, igGetWindowWidth, igSelectable_Bool, ImVec2};
use rfd::FileDialog;
use crate::managers::data::get_app_data;

const FONT_PATH: &[u8] = include_bytes!("../../assets/fonts/Roboto-VariableFont_wdth,wght.ttf");

pub fn roboto_font(font_size: f32) -> FontSource<'static> {
    FontSource::TtfData {
        data: FONT_PATH,
        size_pixels: font_size,
        config: None
    }
}

pub unsafe fn text_center_spacing(text: &str) -> f32 {
    unsafe {
        let window_width = igGetWindowWidth();

        let mut size: ImVec2 = ImVec2::zero();
        igCalcTextSize(&mut size, CString::new(text).unwrap().as_ptr(), null(), true, -1.0);
        let indentation = (window_width - size.x) * 0.5;

        indentation
    }
}

pub fn create_imgui_window<'ui>(ui: &'ui Ui, label: &'ui str) -> Window<'ui, 'ui, &'ui str> {
    ui.window(label)
        .size([800.0, 600.0], imgui::Condition::Never)
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .scrollable(false)
}

pub fn directory_input(ui: &Ui, label: &str, directory_path: &mut PathBuf) {
    let disabled = ui.begin_disabled(true);
    ui.input_text("##Path", &mut directory_path.to_str().unwrap().to_string()).build();
    disabled.end();
    ui.same_line();
    if ui.button("/") {
        let new_directory = FileDialog::new()
            .pick_folder();

        if new_directory.is_some() {
            *directory_path = new_directory.unwrap();
        }
    }
    ui.same_line();
    ui.text(label);
}

pub unsafe fn keyword_list_box(
    ui: &Ui,
    search: &mut String,
    selected_keywords: &mut Vec<String>,
    list_id: &str,
    width: f32,
    height: f32,
) {
    let mut app_data = get_app_data().lock().unwrap();
    let keywords = app_data.keywords.clone();
    let selectable_size = ImVec2::new(width, height / 6.0);

    ui.input_text("Keywords", search).build();

    unsafe {
        if igBeginListBox(CString::new(list_id).unwrap().as_ptr(), ImVec2::new(width, height)) {
            let is_new_keyword = !search.trim().is_empty() && !keywords.iter().any(|e| e.to_lowercase() == search.to_lowercase());

            // Render existing keywords
            for keyword in keywords {
                if !search.trim().is_empty() && !keyword.to_lowercase().contains(&search.to_lowercase()) {
                    continue;
                }

                let is_selected = selected_keywords.iter().any(|e| e.to_lowercase() == keyword.to_lowercase());

                if igSelectable_Bool(CString::new(keyword.clone()).unwrap().as_ptr(), is_selected, 0, selectable_size) {
                    if is_selected {
                        if let Some(pos) = selected_keywords.iter().position(|x| x.to_lowercase() == keyword.to_lowercase()) {
                            selected_keywords.remove(pos);
                        }
                    } else {
                        selected_keywords.push(keyword.clone());
                    }
                    search.clear();
                }
            }

            // Render option to add new keyword
            if is_new_keyword {
                let new_keyword = search.clone();
                if igSelectable_Bool(CString::new(new_keyword.clone()).unwrap().as_ptr(), false, 0, selectable_size) {
                    selected_keywords.push(new_keyword.clone());
                    app_data.keywords.push(new_keyword);
                    search.clear();
                }
            }

            igEndListBox();
        }
    }
}