use crate::models::WorkspaceProfile;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
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
        if let Some(home) = std::env::var_os("USERPROFILE") {
            if let Some(home) = home.to_str() {
                expanded = expanded.replace("~\\", &format!("{home}\\"));
                if expanded == "~" {
                    expanded = home.to_owned();
                }
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
            .map_or(false, |value| value.is_object())
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
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let payload = serde_json::to_string_pretty(value).map_err(|err| err.to_string())?;
    fs::write(path, payload).map_err(|err| err.to_string())?;
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
