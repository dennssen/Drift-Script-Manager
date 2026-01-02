use std::collections::HashSet;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::null;
use imgui::{StyleVar, Ui, Window};
use imgui::sys::{igBeginListBox, igCalcTextSize, igEndListBox, igGetWindowWidth, igSelectable_Bool, ImVec2};
use rfd::FileDialog;
use crate::utils::icons;

pub fn text_center_spacing(text: &str) -> f32 {
    let text_cstring = CString::new(text).expect("Label must not contain null bytes");

    let window_width: f32 = unsafe { igGetWindowWidth() };

    let mut size: ImVec2 = ImVec2::zero();

    unsafe {
        igCalcTextSize(&mut size, text_cstring.as_ptr(), null(), true, -1.0);
    }

    (window_width - size.x) * 0.5
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
    let path_str = directory_path.to_str().unwrap_or("");

    // Always disabled because it is read only. (.read_only() is not obvious enough imo)
    ui.disabled(true, || {
        ui.input_text("##Path", &mut path_str.to_string()).build();
    });

    ui.same_line();
    if ui.button(icons::FOLDER_OPEN) {
        if let Some(new_directory) = FileDialog::new().pick_folder() {
            *directory_path = new_directory;
        }
    }
    ui.same_line();
    ui.text(label);
}

pub fn keyword_list_box(
    ui: &Ui,
    search: &mut String,
    keywords: &[String],
    selected_keywords: &mut Vec<String>,
    list_id: &str,
    width: f32,
    height: f32,
    on_new_keyword: impl FnOnce(String),
) {
    let selectable_size = ImVec2::new(width, height / 6.0);

    ui.input_text("Keywords", search).build();

    let search_lower = search.trim().to_lowercase();
    let selected_lower: HashSet<String> = selected_keywords
        .iter()
        .map(|e| e.to_lowercase())
        .collect();

    list_box(list_id, ImVec2::new(width, height), || {
        // Render existing keywords
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();
            if !search_lower.is_empty() && !keyword_lower.contains(&search_lower) {
                continue;
            }

            let is_selected = selected_lower.contains(&keyword_lower);

            selectable(keyword, is_selected, selectable_size, || {
                if is_selected {
                    selected_keywords.retain(|e| e.to_lowercase() != keyword_lower);
                } else {
                    selected_keywords.push(keyword.clone());
                }
                search.clear();
            });
        }

        let is_new_keyword = !search_lower.is_empty() && !keywords.iter().any(|e| e.to_lowercase() == search_lower);

        // Render option to add new keyword
        if is_new_keyword {
            let new_keyword = search.trim().to_string();
            selectable(&new_keyword.clone(), false, selectable_size, || {
                selected_keywords.push(new_keyword.clone());
                on_new_keyword(new_keyword);
                search.clear();
            });
        }
    });
}

pub fn list_box<F>(label: &str, size: ImVec2, f: F)
where
    F: FnOnce()
{
    let label_cstring = CString::new(label).expect("Label must not contain null bytes");
    let active: bool;
    unsafe {
        active = igBeginListBox(label_cstring.as_ptr(), size)
    }

    if active {
        f();

        unsafe {
            igEndListBox();
        }
    }
}

pub fn selectable<F>(label: &str, is_selected: bool, size: ImVec2, f: F)
where
    F: FnOnce()
{
    let label_cstring = CString::new(label).expect("Label must not contain null bytes");
    let active: bool;
    unsafe {
        active = igSelectable_Bool(label_cstring.as_ptr(), is_selected, 0, size);
    }

    if active {
        f();
    }
}

pub fn main_menu_style<F>(ui: &Ui, f: F)
where 
    F: FnOnce()
{
    let style_push1 = ui.push_style_var(StyleVar::ItemSpacing([8.0, 10.0]));
    let style_push2 = ui.push_style_var(StyleVar::FrameRounding(10.0));
    let style_push3 = ui.push_style_var(StyleVar::FramePadding([10.0, 12.0]));
    
    f();
    
    style_push1.end();
    style_push2.end();
    style_push3.end();
}