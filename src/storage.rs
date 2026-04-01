use crate::models::{AppLocale, AppState, WorkspaceProfile};
use std::fs;
use std::io;
use std::path::PathBuf;

const APP_DIR: &str = "openmcp";
const STATE_FILE: &str = "profiles.json";

fn app_config_dir() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join(APP_DIR)
    } else {
        PathBuf::from(".").join(APP_DIR)
    }
}

pub fn state_file_path() -> PathBuf {
    app_config_dir().join(STATE_FILE)
}

pub fn load_state() -> AppState {
    let path = state_file_path();
    let content = fs::read_to_string(path);

    match content {
        Ok(raw) => serde_json::from_str::<AppState>(&raw).unwrap_or_else(|_| default_state()),
        Err(_) => default_state(),
    }
}

pub fn save_state(state: &AppState) -> io::Result<()> {
    let dir = app_config_dir();
    fs::create_dir_all(&dir)?;

    let payload =
        serde_json::to_string_pretty(state).map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(dir.join(STATE_FILE), payload)
}

fn default_state() -> AppState {
    AppState {
        profiles: vec![WorkspaceProfile::default()],
        selected_profile: 0,
        locale: AppLocale::English,
    }
}
