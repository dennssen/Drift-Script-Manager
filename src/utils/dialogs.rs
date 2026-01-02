use std::io::Error;
use std::sync::{Arc, Mutex};
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use imgui_winit_support::winit::window::Window;

pub static WINDOW: Mutex<Option<Arc<Window>>> = Mutex::new(None);

pub fn error_dialog(title: &str, description: &str, e: &Error) {
    message_dialog(title, format!("{}\n{}", description, e), MessageLevel::Error);
}

pub fn warn_dialog(title: &str, description: &str) {
    message_dialog(title, description.to_string(), MessageLevel::Warning);
}

pub fn info_dialog(title: &str, description: &str) {
    message_dialog(title, description.to_string(), MessageLevel::Info);
}

fn message_dialog(title: &str, description: String, level: MessageLevel) {
    let window_lock = WINDOW.lock().unwrap();
    if let Some(window) = window_lock.as_ref() {
        MessageDialog::new()
            .set_level(level)
            .set_title(title)
            .set_description(description)
            .set_buttons(MessageButtons::Ok)
            .set_parent(window.as_ref())
            .show();
    }
}