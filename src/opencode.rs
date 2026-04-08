use crate::models::WorkspaceProfile;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub enum ConfigAction {
    Install,
    Uninstall,
}

pub fn normalize_config_path(input: &str) -> PathBuf {
    let mut expanded = input.to_owned();

    if cfg!(target_os = "windows") {
        if let Ok(appdata) = std::env::var("APPDATA") {
            expanded = expanded.replace("%APPDATA%", &appdata);
        }
        if let Some(home) = std::env::var_os("USERPROFILE")
            && let Some(home) = home.to_str()
        {
            expanded = expanded.replace("~\\", &format!("{home}\\"));
            if expanded == "~" {
                expanded = home.to_owned();
            }
        }
    } else if let Some(home) = dirs::home_dir().and_then(|v| v.to_str().map(str::to_owned)) {
        if expanded == "~" {
            expanded = home;
        } else if expanded.starts_with("~/") {
            expanded = format!("{home}/{}", expanded.trim_start_matches("~/"));
        }
    }

    PathBuf::from(expanded)
}

pub fn discover_installed_server_ids(target_path: &str) -> Result<Vec<String>, String> {
    let value = load_config(normalize_config_path(target_path))?;
    let mcp = value.get("mcpServers").and_then(Value::as_object);
    let mut ids: Vec<String> = mcp.map_or_else(Vec::new, |map| map.keys().cloned().collect());
    ids.sort();
    Ok(ids)
}

pub fn install_servers(
    profile: &WorkspaceProfile,
    target_path: &str,
    server_ids: &[String],
    respect_enabled: bool,
) -> Result<String, String> {
    apply(
        profile,
        target_path,
        server_ids,
        ConfigAction::Install,
        respect_enabled,
    )
}

pub fn uninstall_servers(
    profile: &WorkspaceProfile,
    target_path: &str,
    server_ids: &[String],
) -> Result<String, String> {
    apply(
        profile,
        target_path,
        server_ids,
        ConfigAction::Uninstall,
        false,
    )
}

pub fn apply(
    profile: &WorkspaceProfile,
    target_path: &str,
    server_ids: &[String],
    action: ConfigAction,
    respect_enabled: bool,
) -> Result<String, String> {
    let ids: BTreeSet<String> = server_ids.iter().map(|id| id.trim().to_owned()).collect();
    if ids.is_empty() {
        return Err("No target server IDs were selected.".to_owned());
    }

    let path = normalize_config_path(target_path);
    let mut value = load_config(path.clone())?;

    let root = value
        .as_object_mut()
        .ok_or_else(|| "OpenCode config should be a JSON object.".to_owned())?;
    if !root.contains_key("mcpServers") {
        root.insert("mcpServers".to_owned(), json!({}));
    }

    let mcp = root
        .get_mut("mcpServers")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "Invalid OpenCode config: mcpServers is not an object.".to_owned())?;

    let mut changed = 0usize;

    match action {
        ConfigAction::Install => {
            for server in &profile.servers {
                if !ids.contains(&server.id) {
                    continue;
                }
                if respect_enabled && !server.enabled {
                    continue;
                }
                mcp.insert(server.id.clone(), server.to_opencode_value());
                changed += 1;
            }
            if changed == 0 {
                return Err("No matching enabled servers were found to install.".to_owned());
            }
        }
        ConfigAction::Uninstall => {
            for id in ids {
                if mcp.remove(&id).is_some() {
                    changed += 1;
                }
            }
            if changed == 0 {
                return Err("No matching server IDs were found to uninstall.".to_owned());
            }
        }
    }

    backup_existing_config(&path)?;
    write_config(&path, &value)?;

    Ok(format!(
        "{} {} {} server(s) in {}",
        match action {
            ConfigAction::Install => "Installed",
            ConfigAction::Uninstall => "Uninstalled",
        },
        changed,
        profile.name,
        path.display()
    ))
}

fn load_config(path: PathBuf) -> Result<Value, String> {
    validate_no_symlink_components(&path)?;

    if !path.exists() {
        return Ok(json!({
            "mcpServers": {}
        }));
    }

    let raw = fs::read_to_string(path).map_err(|err| err.to_string())?;
    if raw.trim().is_empty() {
        return Ok(json!({
            "mcpServers": {}
        }));
    }

    let mut value: Value = serde_json::from_str(&raw).map_err(|err| err.to_string())?;
    if !value.is_object() {
        return Err("OpenCode config should be a JSON object.".to_owned());
    }

    if let Some(root) = value.as_object_mut() {
        if !root.contains_key("mcpServers") {
            root.insert("mcpServers".to_owned(), json!({}));
        } else if !root
            .get("mcpServers")
            .is_some_and(|value| value.is_object())
        {
            return Err("OpenCode config mcpServers key exists but is not an object.".to_owned());
        }
    } else {
        return Err("OpenCode config should be a JSON object.".to_owned());
    }

    Ok(value)
}

fn write_config(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        create_dir_all_secure(parent)?;
    }

    let payload = serde_json::to_string_pretty(value).map_err(|err| err.to_string())?;
    write_string_atomically(path, &payload)?;
    Ok(())
}

fn backup_existing_config(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_millis();
    let backup = path.with_extension(format!("backup-{millis}"));
    fs::copy(path, backup).map_err(|err| err.to_string())?;
    Ok(())
}

fn create_dir_all_secure(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|err| err.to_string())?;
    validate_no_symlink_components(path)?;
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|err| err.to_string())?;
    Ok(())
}

fn write_string_atomically(path: &Path, payload: &str) -> Result<(), String> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "Config file path must end with a valid file name.".to_owned())?;
    let tmp_name = format!(".{file_name}.tmp-{}", std::process::id());
    let tmp_path = path.with_file_name(tmp_name);

    if tmp_path.exists() {
        fs::remove_file(&tmp_path).map_err(|err| err.to_string())?;
    }

    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);

    let mut file = options.open(&tmp_path).map_err(|err| err.to_string())?;
    use std::io::Write as _;
    file.write_all(payload.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(|err| err.to_string())?;
    drop(file);

    fs::rename(&tmp_path, path).map_err(|err| {
        let _ = fs::remove_file(&tmp_path);
        err.to_string()
    })?;

    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(|err| err.to_string())?;

    Ok(())
}

fn validate_no_symlink_components(path: &Path) -> Result<(), String> {
    if let Ok(metadata) = fs::symlink_metadata(path)
        && metadata.file_type().is_symlink()
    {
        return Err(format!(
            "Refusing to follow symbolic links in managed config path: {}",
            path.display()
        ));
    }

    let mut existing_ancestor = path;
    while !existing_ancestor.exists() {
        existing_ancestor = existing_ancestor.parent().ok_or_else(|| {
            format!(
                "Managed config path has no existing parent directory: {}",
                path.display()
            )
        })?;
    }

    let mut resolved = existing_ancestor
        .canonicalize()
        .map_err(|err| err.to_string())?;
    let suffix = path
        .strip_prefix(existing_ancestor)
        .map_err(|err| err.to_string())?;

    for component in suffix.components() {
        resolved.push(component.as_os_str());
        if let Ok(metadata) = fs::symlink_metadata(&resolved)
            && metadata.file_type().is_symlink()
        {
            return Err(format!(
                "Refusing to follow symbolic links in managed config path: {}",
                resolved.display()
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load_config, write_config};
    use serde_json::json;
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
    fn write_config_creates_json_file() {
        let dir = temp_path("config-dir");
        let path = dir.join("config.json");

        write_config(
            &path,
            &json!({"mcpServers": {"demo": {"transport": "stdio"}}}),
        )
        .expect("config should be written");

        let saved = fs::read_to_string(&path).expect("config should exist");
        assert!(saved.contains("\"demo\""));

        fs::remove_dir_all(dir).expect("temp directory should be removed");
    }

    #[test]
    fn load_config_bootstraps_missing_file() {
        let path = temp_path("missing-config").join("config.json");
        let loaded = load_config(path).expect("missing config should bootstrap");
        assert_eq!(loaded, json!({"mcpServers": {}}));
    }

    #[cfg(unix)]
    #[test]
    fn write_config_uses_private_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = temp_path("permissions-dir");
        let path = dir.join("config.json");
        write_config(&path, &json!({"mcpServers": {}})).expect("config should be written");

        let mode = fs::metadata(&path)
            .expect("metadata should exist")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);

        fs::remove_dir_all(dir).expect("temp directory should be removed");
    }

    #[cfg(unix)]
    #[test]
    fn load_config_rejects_symlink_targets() {
        use std::os::unix::fs::symlink;

        let dir = temp_path("symlink-dir");
        let target = dir.join("real-config.json");
        let path = dir.join("config.json");

        fs::create_dir_all(&dir).expect("temp directory should exist");
        fs::write(&target, "{\"mcpServers\":{}}").expect("target should exist");
        symlink(&target, &path).expect("symlink should be created");

        let err = load_config(path).expect_err("symlink targets should be rejected");
        assert!(err.contains("symbolic links"));

        fs::remove_dir_all(dir).expect("temp directory should be removed");
    }
}
