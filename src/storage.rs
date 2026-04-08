use crate::models::{AppLocale, AppState, WorkspaceProfile};
use std::fs;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
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
    create_dir_all_secure(&dir)?;

    let payload =
        serde_json::to_string_pretty(state).map_err(|err| io::Error::other(err.to_string()))?;
    write_string_atomically(&dir.join(STATE_FILE), &payload)
}

fn default_state() -> AppState {
    AppState {
        profiles: vec![WorkspaceProfile::default()],
        selected_profile: 0,
        locale: AppLocale::English,
    }
}

fn create_dir_all_secure(path: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(path)?;
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

fn write_string_atomically(path: &PathBuf, payload: &str) -> io::Result<()> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid state file path"))?;
    let tmp_path = path.with_file_name(format!(".{file_name}.tmp-{}", std::process::id()));

    if tmp_path.exists() {
        fs::remove_file(&tmp_path)?;
    }

    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);

    let mut file = options.open(&tmp_path)?;
    use std::io::Write as _;
    file.write_all(payload.as_bytes())?;
    file.sync_all()?;
    drop(file);

    fs::rename(&tmp_path, path).inspect_err(|_| {
        let _ = fs::remove_file(&tmp_path);
    })?;

    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_string_atomically;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("openmcp-{name}-{unique}-{}", std::process::id()))
    }

    #[test]
    fn atomic_state_write_replaces_file_contents() {
        let dir = temp_path("state-dir");
        let path = dir.join("profiles.json");
        fs::create_dir_all(&dir).expect("temp directory should exist");

        write_string_atomically(&path, "{\"version\":1}").expect("initial write should succeed");
        write_string_atomically(&path, "{\"version\":2}").expect("rewrite should succeed");

        let saved = fs::read_to_string(&path).expect("state file should exist");
        assert_eq!(saved, "{\"version\":2}");

        fs::remove_dir_all(dir).expect("temp directory should be removed");
    }

    #[cfg(unix)]
    #[test]
    fn atomic_state_write_uses_private_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = temp_path("state-permissions");
        let path = dir.join("profiles.json");
        fs::create_dir_all(&dir).expect("temp directory should exist");

        write_string_atomically(&path, "{\"profiles\":[]}").expect("write should succeed");

        let mode = fs::metadata(&path)
            .expect("metadata should exist")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);

        fs::remove_dir_all(dir).expect("temp directory should be removed");
    }
}
