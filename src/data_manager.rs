use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    pub keywords: Vec<String>,
}

static APP_DATA: OnceLock<Mutex<AppData>> = OnceLock::new();

impl AppData {
    pub fn new() -> Self {
        Self {
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
        let appdata_parent_path: PathBuf = dirs::data_dir().unwrap().join("DriftScriptManager");

        if !appdata_parent_path.exists() {
            fs::create_dir(dirs::data_dir().unwrap().join("DriftScriptManager")).unwrap_or_default();
        }

        let appdata_path: PathBuf = appdata_parent_path.join("data.json");

        if !appdata_path.exists() {
            return Self::new();
        }

        let appdata_string = fs::read_to_string(appdata_path);

        if let Err(_) = appdata_string {
            // log the error at some point
            return Self::new();
        }

        let try_parse: Result<AppData, _> = serde_json::from_str(appdata_string.unwrap().as_str());

        if let Err(_) = try_parse {
            // log error
            return Self::new();
        }

        let mut data = try_parse.unwrap();

        // Although unlikely, we make sure there are no empty strings to avoid panics
        data.keywords.retain(|e| !e.trim().is_empty());

        data
    }

    pub fn save(appdata: &Self) {
        let appdata_parent_path: PathBuf = dirs::data_dir().unwrap().join("DriftScriptManager");

        if !appdata_parent_path.exists() {
            fs::create_dir(dirs::data_dir().unwrap().join("DriftScriptManager")).unwrap_or_default();
        }

        let appdata_string = serde_json::to_string_pretty(appdata);

        if let Err(_) = appdata_string {
            // log
            return;
        }

        let write_result = fs::write(appdata_parent_path.join("data.json"), appdata_string.unwrap());

        if let Err(_) = write_result {
            // log
        }
    }
}

pub fn get_app_data() -> &'static Mutex<AppData> {
    APP_DATA.get_or_init(|| Mutex::new(AppData::load_or_create()))
}