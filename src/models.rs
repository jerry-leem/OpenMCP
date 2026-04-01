use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppLocale {
    English,
    Korean,
}

impl Default for AppLocale {
    fn default() -> Self {
        Self::English
    }
}

// locale-specific label text is intentionally handled in UI translation tables

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileScope {
    Global,
    Project,
}

impl ProfileScope {
    pub fn label(self, locale: AppLocale) -> &'static str {
        match (self, locale) {
            (Self::Global, AppLocale::English) => "Global",
            (Self::Project, AppLocale::English) => "Project",
            (Self::Global, AppLocale::Korean) => "전역",
            (Self::Project, AppLocale::Korean) => "프로젝트",
        }
    }

    pub fn hint(self, locale: AppLocale) -> &'static str {
        match (self, locale) {
            (Self::Global, AppLocale::English) => {
                "Use this when the same MCP server should be available across all OpenCode workspaces."
            }
            (Self::Project, AppLocale::English) => {
                "Use this when the MCP server is tied to one repository or local project."
            }
            (Self::Global, AppLocale::Korean) => {
                "하나의 MCP 서버를 모든 OpenCode 작업 공간에서 공유해야 할 때 사용하세요."
            }
            (Self::Project, AppLocale::Korean) => {
                "특정 저장소나 로컬 프로젝트에만 필요한 MCP 서버를 설정할 때 사용하세요."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportKind {
    Stdio,
    Sse,
    Http,
}

impl TransportKind {
    pub fn label(self, locale: AppLocale) -> &'static str {
        match (self, locale) {
            (Self::Stdio, AppLocale::English) => "stdio",
            (Self::Sse, AppLocale::English) => "sse",
            (Self::Http, AppLocale::English) => "http",
            (Self::Stdio, AppLocale::Korean) => "stdio",
            (Self::Sse, AppLocale::Korean) => "sse",
            (Self::Http, AppLocale::Korean) => "http",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub id: String,
    pub transport: TransportKind,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub env: BTreeMap<String, String>,
    pub url: String,
    pub notes: String,
    pub enabled: bool,
}

impl McpServer {
    pub fn to_opencode_value(&self) -> Value {
        match self.transport {
            TransportKind::Stdio => {
                let mut payload = Map::new();
                payload.insert("transport".to_owned(), json!("stdio"));
                payload.insert("command".to_owned(), json!(self.command));

                if !self.args.is_empty() {
                    payload.insert("args".to_owned(), json!(self.args));
                }

                if !self.cwd.trim().is_empty() {
                    payload.insert("cwd".to_owned(), json!(self.cwd));
                }

                let env = self
                    .env
                    .iter()
                    .filter(|(key, _)| !key.trim().is_empty())
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<BTreeMap<String, String>>();
                if !env.is_empty() {
                    payload.insert("env".to_owned(), json!(env));
                }

                if !self.notes.trim().is_empty() {
                    payload.insert("_notes".to_owned(), json!(self.notes));
                }

                Value::Object(payload)
            }
            TransportKind::Sse => json!({
                "transport": "sse",
                "url": self.url,
                "_notes": self.notes,
            }),
            TransportKind::Http => json!({
                "transport": "http",
                "url": self.url,
                "_notes": self.notes,
            }),
        }
    }
}

impl Default for McpServer {
    fn default() -> Self {
        let mut env = BTreeMap::new();
        env.insert("OPENAI_API_KEY".to_owned(), "".to_owned());

        Self {
            id: "filesystem".to_owned(),
            transport: TransportKind::Stdio,
            command: "npx".to_owned(),
            args: vec![
                "-y".to_owned(),
                "@modelcontextprotocol/server-filesystem".to_owned(),
                ".".to_owned(),
            ],
            cwd: ".".to_owned(),
            env,
            url: String::new(),
            notes: "Example profile. Replace this with the MCP server you actually want to expose in OpenCode.".to_owned(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceProfile {
    pub name: String,
    pub scope: ProfileScope,
    pub target_hint: String,
    pub open_code_path: String,
    pub servers: Vec<McpServer>,
}

impl Default for WorkspaceProfile {
    fn default() -> Self {
        Self {
            name: "Default OpenCode Profile".to_owned(),
            scope: ProfileScope::Project,
            target_hint: "Paste the generated JSON into the MCP section used by your OpenCode workspace or app settings.".to_owned(),
            open_code_path: default_opencode_path(),
            servers: vec![McpServer::default()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub profiles: Vec<WorkspaceProfile>,
    pub selected_profile: usize,
    #[serde(default)]
    pub locale: AppLocale,
}

pub fn default_opencode_path() -> String {
    if cfg!(target_os = "macos") {
        "~/Library/Application Support/OpenCode/config.json".to_owned()
    } else if cfg!(target_os = "windows") {
        r"%APPDATA%\OpenCode\config.json".to_owned()
    } else {
        "~/.config/opencode/config.json".to_owned()
    }
}
