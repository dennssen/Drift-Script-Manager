use std::fmt::Display;
use std::io::Error;
use std::sync::{Arc, Mutex};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use imgui_winit_support::winit::window::Window;

pub static WINDOW: Mutex<Option<Arc<Window>>> = Mutex::new(None);

pub fn error_dialog<S>(title: S, description: S, e: &Error)
where
    S: Into<String> + Display
{
    message_dialog(title, format!("{}\n{}", description, e), MessageLevel::Error, MessageButtons::Ok);
}

pub fn warn_dialog<S>(title: S, description: S)
where
    S: Into<String> + Display
{
    message_dialog(title, description, MessageLevel::Warning, MessageButtons::Ok);
}

pub fn info_dialog<S>(title: S, description: S)
where
    S: Into<String> + Display
{
    message_dialog(title, description, MessageLevel::Info, MessageButtons::Ok);
}

pub fn option_dialog<S>(title: S, description: S) -> MessageDialogResult
where
    S: Into<String> + Display
{
    message_dialog(title, description, MessageLevel::Info, MessageButtons::YesNo)
}

fn message_dialog<S, D>(title: S, description: D, level: MessageLevel, buttons: MessageButtons) -> MessageDialogResult
where
    S: Into<String>,
    D: Into<String>
{
    let mut message_dialog = MessageDialog::new()
        .set_title(title)
        .set_description(description)
        .set_level(level)
        .set_buttons(buttons);

    let window_lock = WINDOW.lock().unwrap();
    if let Some(window) = window_lock.as_ref() {
        message_dialog = message_dialog.set_parent(window.as_ref());
    }

    message_dialog.show()
}