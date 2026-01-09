use std::fmt::Display;
use std::io::Error;
use std::sync::{Arc, Mutex};
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use imgui_winit_support::winit::window::Window;

pub static WINDOW: Mutex<Option<Arc<Window>>> = Mutex::new(None);

pub fn error_dialog<S>(title: S, description: S, e: &Error)
where
    S: Into<String> + Display
{
    message_dialog(title, format!("{}\n{}", description, e), MessageLevel::Error);
}

pub fn warn_dialog<S>(title: S, description: S)
where
    S: Into<String> + Display
{
    message_dialog(title, description, MessageLevel::Warning);
}

pub fn info_dialog<S>(title: S, description: S)
where
    S: Into<String> + Display
{
    message_dialog(title, description, MessageLevel::Info);
}

fn message_dialog<S, D>(title: S, description: D, level: MessageLevel)
where
    S: Into<String>,
    D: Into<String>
{
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