use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use capitalize::Capitalize;
use serde::{Deserialize, Serialize};
use winit::dpi::PhysicalPosition;
use crate::utils::dialogs::error_dialog;
use crate::utils::error_helper::json_error_to_io;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    #[serde(default)]
    pub outer_window_pos: Option<PhysicalPosition<i32>>,
    pub keywords: Vec<String>,
}

static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

static APP_DATA: OnceLock<Mutex<AppData>> = OnceLock::new();

impl AppData {
    pub fn get_dir() -> &'static PathBuf {
        APP_DATA_DIR.get_or_init(|| {
            let dir = dirs::data_dir().unwrap().join("DriftScriptManager");
            if !dir.exists() {
                fs::create_dir(&dir).unwrap_or_default();
            }

            dir
        })
    }

    pub fn new() -> Self {
        Self {
            outer_window_pos: None,
            keywords: vec![
                String::from("POV"),
                String::from("Framework"),
                String::from("Freecam"),
                String::from("Replay Support"),
                String::from("Keybinds"),
                String::from("Controller"),
                String::from("ZDrift"),
                String::from("Driftball"),
                String::from("Driftblitz"),
                String::from("Go Pro"),
                String::from("Content Creation")
            ]
        }
    }

    pub fn load_or_create() -> Self {
        let appdata_dir: &PathBuf = Self::get_dir();

        assert!(appdata_dir.exists());

        let appdata_path: PathBuf = appdata_dir.join("data.json");

        if !appdata_path.exists() {
            return Self::new();
        }

        let appdata_string = fs::read_to_string(appdata_path);

        if let Err(e) = appdata_string {
            error_dialog("AppData Failure", "Failed to read saved data. Created new save data.", &e);
            return Self::new();
        }

        let try_parse: Result<AppData, _> = serde_json::from_str(appdata_string.unwrap().as_str());

        if let Err(e) = try_parse {
            let e = json_error_to_io(&e);
            error_dialog("AppData Failure", "Failed to parse save data. Created new save data.", &e);
            return Self::new();
        }

        let mut data = try_parse.unwrap();

        // Although unlikely, we make sure there are no empty strings to avoid panics
        data.keywords.retain(|e| !e.trim().is_empty());

        data
    }

    pub fn update_keywords(&mut self, keywords: &[String]) {

        for keyword in keywords {
            if keyword.is_empty() || self.keywords.iter().any(|e| e.to_lowercase() == keyword.to_lowercase()){
                continue;
            }

            // Capitalize to try and match the default keywords
            self.keywords.push(keyword.capitalize_first_only());
        }
    }

    pub fn save(&self) {
        let appdata_parent_path: PathBuf = dirs::data_dir().unwrap().join("DriftScriptManager");

        if !appdata_parent_path.exists() {
            fs::create_dir(dirs::data_dir().unwrap().join("DriftScriptManager")).unwrap_or_default();
        }

        let appdata_string = serde_json::to_string_pretty(self);

        if let Err(e) = appdata_string {
            let e = json_error_to_io(&e);
            error_dialog("Serialize AppData Failure", "Failed to serialize save data", &e);
            return;
        }

        let write_result = fs::write(appdata_parent_path.join("data.json"), appdata_string.unwrap());

        if let Err(e) = write_result {
            error_dialog("Write AppData Failure", "Failed to write save data", &e);
        }
    }
}

pub fn get_app_data() -> &'static Mutex<AppData> {
    APP_DATA.get_or_init(|| Mutex::new(AppData::load_or_create()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_keywords_no_empty() {
        let mut app_data = AppData::new();
        app_data.keywords = vec![String::from("Existing keyword")];

        let new_keywords = vec![String::new()];

        app_data.update_keywords(&new_keywords);

        let found_empty = app_data.keywords.iter().any(|e| e.is_empty());

        assert!(!found_empty, "Found empty keyword");
    }

    #[test]
    fn test_update_keywords_no_duplicates() {
        let mut app_data = AppData::new();
        app_data.keywords = vec![String::from("Existing keyword")];

        let new_keywords = vec![String::from("Existing keyword")];

        app_data.update_keywords(&new_keywords);

        assert!(app_data.keywords.len() < 2, "Accepted duplicate");
    }
}