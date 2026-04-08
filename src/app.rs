use crate::models::{
    AppLocale, AppState, McpServer, ProfileScope, TransportKind, WorkspaceProfile,
};
use crate::opencode;
use crate::storage;
use eframe::egui::{
    self, Align, Button, Color32, Context, Layout, RichText, ScrollArea, SidePanel, TextEdit,
    TopBottomPanel, Ui,
};
use serde_json::{Map, json};
use std::collections::BTreeSet;

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum UiText {
    AppTitle,
    AppSubtitle,
    BtnSave,
    BtnUninstallEnabled,
    BtnInstallEnabled,
    BtnLoadConfig,
    BtnCopySetup,
    BtnCopyJson,
    Profiles,
    ProfilesHint,
    BtnNew,
    BtnDuplicate,
    BtnDelete,
    ProfileSection,
    ScopeLabel,
    ScopeHintLabel,
    OpenCodePathLabel,
    OpenCodePathHint,
    OperatorNoteLabel,
    OperatorNoteHint,
    McpServersSection,
    LanguageSection,
    LanguageValueLabel,
    ServerCountLabel,
    TransportLabel,
    TransportStdioLabel,
    TransportSseLabel,
    TransportHttpLabel,
    ServerCardInstalled,
    ServerCardNotInstalled,
    ServerIdLabel,
    CommandLabel,
    ArgsLabel,
    WorkingDirectoryLabel,
    EndpointLabel,
    EnvVarsLabel,
    AddEnvVarButton,
    NotesLabel,
    RemoveServerButton,
    AddServerButton,
    GeneratedConfigTitle,
    GeneratedConfigHint,
    ValidationPassed,
    ValidationFixHint,
    OpenCodeGuideTitle,
    FlowTitle,
    FlowStep1,
    FlowStep2,
    FlowStep3,
    FlowStep4,
    CopyJsonAgain,
    FlowScopePrefix,
    BtnInstallServerCard,
    BtnUninstallServerCard,
    NotesHint,
    ProfileNameHint,
    ConfigManagerTitle,
    ReloadInstalledStateButton,
    ManagedPathLabel,
    NoServerInConfig,
    InstalledServerIds,
    StatusReady,
    StatusUnsaved,
    StatusLoadedConfigState,
    StatusConfigReadFailed,
    StatusInstallFailed,
    StatusUninstallFailed,
    StatusEmptyIdFailed,
    StatusOperationFailed,
    StatusSaveFailed,
    StatusSaved,
    StatusCopiedJson,
    StatusCopiedSetup,
    StatusProfileCreated,
    StatusProfileDuplicated,
    StatusProfileDeleteBlocked,
    StatusProfileDeleted,
    StatusServerRemoved,
    StatusServerAdded,
    StatusLanguageChanged,
}

impl AppLocale {
    fn t(self, key: UiText) -> &'static str {
        match (self, key) {
            (Self::English, UiText::AppTitle) => "OpenMCP",
            (Self::English, UiText::AppSubtitle) => {
                "Build OpenCode MCP configs without hand-editing JSON."
            }
            (Self::English, UiText::BtnSave) => "Save",
            (Self::English, UiText::BtnUninstallEnabled) => "Uninstall Enabled",
            (Self::English, UiText::BtnInstallEnabled) => "Install Enabled",
            (Self::English, UiText::BtnLoadConfig) => "Load Config",
            (Self::English, UiText::BtnCopySetup) => "Copy Setup",
            (Self::English, UiText::BtnCopyJson) => "Copy JSON",
            (Self::English, UiText::Profiles) => "Profiles",
            (Self::English, UiText::ProfilesHint) => {
                "Separate project-specific MCP wiring from global setups."
            }
            (Self::English, UiText::BtnNew) => "New",
            (Self::English, UiText::BtnDuplicate) => "Duplicate",
            (Self::English, UiText::BtnDelete) => "Delete",
            (Self::English, UiText::ProfileSection) => "Profile",
            (Self::English, UiText::ScopeLabel) => "Scope",
            (Self::English, UiText::ScopeHintLabel) => "Project or global scope",
            (Self::English, UiText::OpenCodePathLabel) => "OpenCode target path hint",
            (Self::English, UiText::OpenCodePathHint) => {
                "Where you expect to paste or merge the config"
            }
            (Self::English, UiText::OperatorNoteLabel) => "Operator note",
            (Self::English, UiText::OperatorNoteHint) => {
                "Explain where or how this JSON should be applied"
            }
            (Self::English, UiText::McpServersSection) => "MCP Servers",
            (Self::English, UiText::LanguageSection) => "Language",
            (Self::English, UiText::LanguageValueLabel) => "Select language",
            (Self::English, UiText::ServerCountLabel) => "Server",
            (Self::English, UiText::TransportLabel) => "Transport",
            (Self::English, UiText::TransportStdioLabel) => "stdio",
            (Self::English, UiText::TransportSseLabel) => "sse",
            (Self::English, UiText::TransportHttpLabel) => "http",
            (Self::English, UiText::ServerCardInstalled) => "Installed",
            (Self::English, UiText::ServerCardNotInstalled) => "Not installed",
            (Self::English, UiText::ServerIdLabel) => "Server id",
            (Self::English, UiText::CommandLabel) => "Command",
            (Self::English, UiText::ArgsLabel) => "Arguments",
            (Self::English, UiText::WorkingDirectoryLabel) => "Working directory",
            (Self::English, UiText::EndpointLabel) => "Endpoint URL",
            (Self::English, UiText::EnvVarsLabel) => "Environment variables",
            (Self::English, UiText::AddEnvVarButton) => "Add Env Var",
            (Self::English, UiText::NotesLabel) => "Notes",
            (Self::English, UiText::RemoveServerButton) => "Remove",
            (Self::English, UiText::AddServerButton) => "Add MCP Server",
            (Self::English, UiText::GeneratedConfigTitle) => "Generated Config",
            (Self::English, UiText::GeneratedConfigHint) => {
                "Paste this `mcpServers` block into the place OpenCode expects."
            }
            (Self::English, UiText::ValidationPassed) => "Validation passed",
            (Self::English, UiText::ValidationFixHint) => "Fix these issues before exporting",
            (Self::English, UiText::OpenCodeGuideTitle) => "OpenCode Guide",
            (Self::English, UiText::FlowTitle) => "Suggested flow",
            (Self::English, UiText::FlowStep1) => {
                "1. Install the MCP server runtime or binary used by the command."
            }
            (Self::English, UiText::FlowStep2) => {
                "2. Verify the command runs outside OpenCode first."
            }
            (Self::English, UiText::FlowStep3) => {
                "3. Copy the generated JSON and merge it into your OpenCode config."
            }
            (Self::English, UiText::FlowStep4) => {
                "4. Reload OpenCode and confirm the server appears."
            }
            (Self::English, UiText::FlowScopePrefix) => "Recommended scope",
            (Self::English, UiText::BtnInstallServerCard) => "Install to OpenCode",
            (Self::English, UiText::BtnUninstallServerCard) => "Uninstall from OpenCode",
            (Self::English, UiText::NotesHint) => {
                "Any reminder about installation, auth, or expected capabilities"
            }
            (Self::English, UiText::ProfileNameHint) => "Profile name",
            (Self::English, UiText::CopyJsonAgain) => "Copy JSON Again",
            (Self::English, UiText::ConfigManagerTitle) => "OpenCode Config Manager",
            (Self::English, UiText::ReloadInstalledStateButton) => "Reload Installed State",
            (Self::English, UiText::ManagedPathLabel) => "Managed path",
            (Self::English, UiText::NoServerInConfig) => "No MCP servers found in config.",
            (Self::English, UiText::InstalledServerIds) => "Current installed server ids:",
            (Self::English, UiText::StatusReady) => "Ready",
            (Self::English, UiText::StatusLoadedConfigState) => "Loaded OpenCode config state",
            (Self::English, UiText::StatusConfigReadFailed) => "Config read failed",
            (Self::English, UiText::StatusInstallFailed) => "Install failed",
            (Self::English, UiText::StatusUninstallFailed) => "Uninstall failed",
            (Self::English, UiText::StatusEmptyIdFailed) => {
                "Install/Uninstall failed: server id is empty."
            }
            (Self::English, UiText::StatusOperationFailed) => "Operation failed",
            (Self::English, UiText::StatusSaveFailed) => "Save failed",
            (Self::English, UiText::StatusSaved) => "Saved",
            (Self::English, UiText::StatusCopiedJson) => "Generated JSON copied to clipboard",
            (Self::English, UiText::StatusCopiedSetup) => "Setup instructions copied to clipboard",
            (Self::English, UiText::StatusProfileCreated) => "New profile created",
            (Self::English, UiText::StatusProfileDuplicated) => "Profile duplicated",
            (Self::English, UiText::StatusProfileDeleteBlocked) => {
                "At least one profile must remain"
            }
            (Self::English, UiText::StatusProfileDeleted) => "Profile deleted",
            (Self::English, UiText::StatusServerRemoved) => "Server removed",
            (Self::English, UiText::StatusServerAdded) => "Server added",
            (Self::English, UiText::StatusLanguageChanged) => "Language changed",
            (Self::English, UiText::StatusUnsaved) => "Unsaved changes",

            (Self::Korean, UiText::AppTitle) => "OpenMCP",
            (Self::Korean, UiText::AppSubtitle) => {
                "OpenCode MCP 설정을 JSON을 직접 편집하지 않고 생성하세요."
            }
            (Self::Korean, UiText::BtnSave) => "저장",
            (Self::Korean, UiText::BtnUninstallEnabled) => "활성 서버 제거",
            (Self::Korean, UiText::BtnInstallEnabled) => "활성 서버 설치",
            (Self::Korean, UiText::BtnLoadConfig) => "설정 다시 불러오기",
            (Self::Korean, UiText::BtnCopySetup) => "설치 안내 복사",
            (Self::Korean, UiText::BtnCopyJson) => "JSON 복사",
            (Self::Korean, UiText::Profiles) => "프로필",
            (Self::Korean, UiText::ProfilesHint) => {
                "프로젝트별 MCP 설정과 전역 설정을 분리해서 관리할 수 있습니다."
            }
            (Self::Korean, UiText::BtnNew) => "새로 만들기",
            (Self::Korean, UiText::BtnDuplicate) => "복제",
            (Self::Korean, UiText::BtnDelete) => "삭제",
            (Self::Korean, UiText::ProfileSection) => "프로필",
            (Self::Korean, UiText::ScopeLabel) => "적용 범위",
            (Self::Korean, UiText::ScopeHintLabel) => "프로젝트 또는 전역",
            (Self::Korean, UiText::OpenCodePathLabel) => "OpenCode 경로 힌트",
            (Self::Korean, UiText::OpenCodePathHint) => {
                "설정을 붙여넣거나 병합할 경로를 입력하세요"
            }
            (Self::Korean, UiText::OperatorNoteLabel) => "운영 메모",
            (Self::Korean, UiText::OperatorNoteHint) => "이 JSON 적용 위치/방식을 설명하세요",
            (Self::Korean, UiText::McpServersSection) => "MCP 서버",
            (Self::Korean, UiText::LanguageSection) => "언어",
            (Self::Korean, UiText::LanguageValueLabel) => "언어 선택",
            (Self::Korean, UiText::ServerCountLabel) => "서버",
            (Self::Korean, UiText::TransportLabel) => "전송 방식",
            (Self::Korean, UiText::TransportStdioLabel) => "stdio",
            (Self::Korean, UiText::TransportSseLabel) => "sse",
            (Self::Korean, UiText::TransportHttpLabel) => "http",
            (Self::Korean, UiText::ServerCardInstalled) => "설치됨",
            (Self::Korean, UiText::ServerCardNotInstalled) => "미설치",
            (Self::Korean, UiText::ServerIdLabel) => "서버 ID",
            (Self::Korean, UiText::CommandLabel) => "실행 명령",
            (Self::Korean, UiText::ArgsLabel) => "인자",
            (Self::Korean, UiText::WorkingDirectoryLabel) => "작업 디렉터리",
            (Self::Korean, UiText::EndpointLabel) => "엔드포인트 URL",
            (Self::Korean, UiText::EnvVarsLabel) => "환경 변수",
            (Self::Korean, UiText::AddEnvVarButton) => "환경 변수 추가",
            (Self::Korean, UiText::NotesLabel) => "메모",
            (Self::Korean, UiText::RemoveServerButton) => "제거",
            (Self::Korean, UiText::AddServerButton) => "MCP 서버 추가",
            (Self::Korean, UiText::GeneratedConfigTitle) => "생성된 설정",
            (Self::Korean, UiText::GeneratedConfigHint) => {
                "OpenCode가 기대하는 위치에 `mcpServers` 블록을 붙여 넣으세요."
            }
            (Self::Korean, UiText::ValidationPassed) => "유효성 검사 통과",
            (Self::Korean, UiText::ValidationFixHint) => "내보내기 전 오류를 먼저 수정하세요",
            (Self::Korean, UiText::OpenCodeGuideTitle) => "OpenCode 가이드",
            (Self::Korean, UiText::FlowTitle) => "권장 동작 순서",
            (Self::Korean, UiText::FlowStep1) => {
                "1. 명령어에 사용되는 MCP 실행 파일/런타임을 먼저 설치하세요."
            }
            (Self::Korean, UiText::FlowStep2) => {
                "2. OpenCode 외부에서 명령이 정상 동작하는지 먼저 확인하세요."
            }
            (Self::Korean, UiText::FlowStep3) => {
                "3. 생성된 JSON을 복사해 OpenCode 설정에 병합하세요."
            }
            (Self::Korean, UiText::FlowStep4) => {
                "4. OpenCode를 다시 로드하고 서버 등록을 확인하세요."
            }
            (Self::Korean, UiText::FlowScopePrefix) => "권장 범위",
            (Self::Korean, UiText::BtnInstallServerCard) => "OpenCode에 설치",
            (Self::Korean, UiText::BtnUninstallServerCard) => "OpenCode에서 제거",
            (Self::Korean, UiText::NotesHint) => "설치, 인증, 기대 동작에 대한 메모를 입력하세요",
            (Self::Korean, UiText::ProfileNameHint) => "프로필 이름",
            (Self::Korean, UiText::CopyJsonAgain) => "JSON 다시 복사",
            (Self::Korean, UiText::ConfigManagerTitle) => "OpenCode 설정 관리자",
            (Self::Korean, UiText::ReloadInstalledStateButton) => "설치 상태 새로고침",
            (Self::Korean, UiText::ManagedPathLabel) => "관리 대상 경로",
            (Self::Korean, UiText::NoServerInConfig) => "설정 파일에서 MCP 서버를 찾지 못했습니다.",
            (Self::Korean, UiText::InstalledServerIds) => "현재 설치된 서버 ID:",
            (Self::Korean, UiText::StatusReady) => "준비됨",
            (Self::Korean, UiText::StatusLoadedConfigState) => "OpenCode 설정 상태를 읽어왔습니다",
            (Self::Korean, UiText::StatusConfigReadFailed) => "설정 읽기 실패",
            (Self::Korean, UiText::StatusInstallFailed) => "설치 실패",
            (Self::Korean, UiText::StatusUninstallFailed) => "삭제 실패",
            (Self::Korean, UiText::StatusEmptyIdFailed) => {
                "설치/삭제 실패: 서버 ID가 비어 있습니다."
            }
            (Self::Korean, UiText::StatusOperationFailed) => "작업 실패",
            (Self::Korean, UiText::StatusSaveFailed) => "저장 실패",
            (Self::Korean, UiText::StatusSaved) => "저장됨",
            (Self::Korean, UiText::StatusCopiedJson) => "JSON이 클립보드에 복사됨",
            (Self::Korean, UiText::StatusCopiedSetup) => "설치 안내가 클립보드에 복사됨",
            (Self::Korean, UiText::StatusProfileCreated) => "새 프로필이 생성됨",
            (Self::Korean, UiText::StatusProfileDuplicated) => "프로필이 복제됨",
            (Self::Korean, UiText::StatusProfileDeleteBlocked) => {
                "최소 하나의 프로필은 남겨야 합니다"
            }
            (Self::Korean, UiText::StatusProfileDeleted) => "프로필이 삭제됨",
            (Self::Korean, UiText::StatusServerRemoved) => "서버가 삭제됨",
            (Self::Korean, UiText::StatusServerAdded) => "서버가 추가됨",
            (Self::Korean, UiText::StatusLanguageChanged) => "언어가 변경됨",
            (Self::Korean, UiText::StatusUnsaved) => "저장되지 않음",
        }
    }
}

enum ServerConfigAction {
    Install(usize),
    Uninstall(usize),
}

pub struct OpenMcpApp {
    state: AppState,
    status: String,
    dirty: bool,
    managed_server_ids: Vec<String>,
}

impl OpenMcpApp {
    fn locale(&self) -> AppLocale {
        self.state.locale
    }

    fn txt(&self, key: UiText) -> &'static str {
        self.locale().t(key)
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            state: storage::load_state(),
            status: String::new(),
            dirty: false,
            managed_server_ids: Vec::new(),
        };
        app.status = app.txt(UiText::StatusReady).to_owned();
        app.refresh_managed_servers();
        app
    }

    fn refresh_managed_servers(&mut self) {
        let profile = self.selected_profile().clone();
        match opencode::discover_installed_server_ids(&profile.open_code_path) {
            Ok(ids) => {
                self.managed_server_ids = ids;
                self.status = self.txt(UiText::StatusLoadedConfigState).to_owned();
            }
            Err(err) => {
                self.status = format!("{}: {err}", self.txt(UiText::StatusConfigReadFailed));
                self.managed_server_ids = Vec::new();
            }
        }
    }

    fn target_server_ids_for_action(&self) -> Vec<String> {
        let profile = self.selected_profile();
        profile
            .servers
            .iter()
            .filter(|server| server.enabled)
            .map(|server| server.id.clone())
            .collect()
    }

    fn install_enabled_servers(&mut self) {
        let profile = self.selected_profile().clone();
        let ids = self.target_server_ids_for_action();
        match opencode::install_servers(&profile, &profile.open_code_path, &ids, true) {
            Ok(message) => {
                self.status = message;
                self.refresh_managed_servers();
                self.dirty = false;
            }
            Err(message) => {
                self.status = format!("{}: {message}", self.txt(UiText::StatusInstallFailed));
            }
        }
    }

    fn uninstall_enabled_servers(&mut self) {
        let profile = self.selected_profile().clone();
        let mut ids = Vec::new();
        for server_id in &self.managed_server_ids {
            let exists = profile.servers.iter().any(|server| server.id == *server_id);
            if exists {
                ids.push(server_id.clone());
            }
        }

        match opencode::uninstall_servers(&profile, &profile.open_code_path, &ids) {
            Ok(message) => {
                self.status = message;
                self.refresh_managed_servers();
                self.dirty = false;
            }
            Err(message) => {
                self.status = format!("{}: {message}", self.txt(UiText::StatusUninstallFailed));
            }
        }
    }

    fn apply_single_server_action(&mut self, action: ServerConfigAction) {
        let profile = self.selected_profile().clone();
        let id = match action {
            ServerConfigAction::Install(index) | ServerConfigAction::Uninstall(index) => {
                profile.servers.get(index).map(|server| server.id.clone())
            }
        };

        if let Some(id) = id {
            if id.trim().is_empty() {
                self.status = self.txt(UiText::StatusEmptyIdFailed).to_owned();
                return;
            }
            let ids = vec![id];
            let result = match action {
                ServerConfigAction::Install(_) => {
                    opencode::install_servers(&profile, &profile.open_code_path, &ids, false)
                }
                ServerConfigAction::Uninstall(_) => {
                    opencode::uninstall_servers(&profile, &profile.open_code_path, &ids)
                }
            };

            match result {
                Ok(message) => {
                    self.status = message;
                    self.refresh_managed_servers();
                }
                Err(message) => {
                    self.status = format!("{}: {message}", self.txt(UiText::StatusOperationFailed));
                }
            }
        }
    }

    fn selected_profile_mut(&mut self) -> &mut WorkspaceProfile {
        let index = self
            .state
            .selected_profile
            .min(self.state.profiles.len().saturating_sub(1));
        &mut self.state.profiles[index]
    }

    fn selected_profile(&self) -> &WorkspaceProfile {
        let index = self
            .state
            .selected_profile
            .min(self.state.profiles.len().saturating_sub(1));
        &self.state.profiles[index]
    }

    fn save(&mut self) {
        match storage::save_state(&self.state) {
            Ok(()) => {
                self.status = format!(
                    "{} {}",
                    self.txt(UiText::StatusSaved),
                    storage::state_file_path().display()
                );
                self.dirty = false;
            }
            Err(err) => {
                self.status = format!("{}: {err}", self.txt(UiText::StatusSaveFailed));
            }
        }
    }

    fn add_profile(&mut self) {
        self.state.profiles.push(WorkspaceProfile {
            name: format!("Profile {}", self.state.profiles.len() + 1),
            ..WorkspaceProfile::default()
        });
        self.state.selected_profile = self.state.profiles.len() - 1;
        self.refresh_managed_servers();
        self.dirty = true;
        self.status = self.txt(UiText::StatusProfileCreated).to_owned();
    }

    fn duplicate_profile(&mut self) {
        let mut clone = self.selected_profile().clone();
        clone.name = format!("{} Copy", clone.name);
        self.state.profiles.push(clone);
        self.state.selected_profile = self.state.profiles.len() - 1;
        self.refresh_managed_servers();
        self.dirty = true;
        self.status = self.txt(UiText::StatusProfileDuplicated).to_owned();
    }

    fn delete_selected_profile(&mut self) {
        if self.state.profiles.len() <= 1 {
            self.status = self.txt(UiText::StatusProfileDeleteBlocked).to_owned();
            return;
        }

        self.state.profiles.remove(self.state.selected_profile);
        self.state.selected_profile = self
            .state
            .selected_profile
            .min(self.state.profiles.len().saturating_sub(1));
        self.refresh_managed_servers();
        self.dirty = true;
        self.status = self.txt(UiText::StatusProfileDeleted).to_owned();
    }

    fn generated_json(&self) -> String {
        let profile = self.selected_profile();
        let mut servers = Map::new();

        for server in &profile.servers {
            if !server.enabled {
                continue;
            }
            servers.insert(server.id.clone(), server.to_opencode_value());
        }

        serde_json::to_string_pretty(&json!({
            "mcpServers": servers
        }))
        .unwrap_or_else(|_| "{}".to_owned())
    }

    fn validation_messages(&self) -> Vec<String> {
        let profile = self.selected_profile();
        let mut messages = Vec::new();
        let mut seen = BTreeSet::new();

        if profile.name.trim().is_empty() {
            messages.push(if self.state.locale == AppLocale::Korean {
                "프로필 이름이 필요합니다.".to_owned()
            } else {
                "Profile name is required.".to_owned()
            });
        }

        for (index, server) in profile.servers.iter().enumerate() {
            let slot = index + 1;

            if server.id.trim().is_empty() {
                messages.push(if self.state.locale == AppLocale::Korean {
                    format!("서버 {slot}: id가 필요합니다.")
                } else {
                    format!("Server {slot}: id is required.")
                });
            }

            if !server
                .id
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
            {
                messages.push(if self.state.locale == AppLocale::Korean {
                    format!("서버 {slot}: ID는 영문, 숫자, '-', '_'만 사용할 수 있습니다.")
                } else {
                    format!("Server {slot}: id should use letters, numbers, '-' or '_'.")
                });
            }

            if !server.id.trim().is_empty() && !seen.insert(server.id.clone()) {
                messages.push(if self.state.locale == AppLocale::Korean {
                    format!("서버 {slot}: ID '{}'가 중복됩니다.", server.id)
                } else {
                    format!("Server {slot}: duplicate id '{}'.", server.id)
                });
            }

            match server.transport {
                TransportKind::Stdio => {
                    if server.command.trim().is_empty() {
                        messages.push(if self.state.locale == AppLocale::Korean {
                            format!("서버 {slot}: stdio는 command가 필요합니다.")
                        } else {
                            format!("Server {slot}: command is required for stdio.")
                        });
                    }
                }
                TransportKind::Sse | TransportKind::Http => {
                    if server.url.trim().is_empty() {
                        messages.push(if self.state.locale == AppLocale::Korean {
                            format!("서버 {slot}: 네트워크 전송은 URL이 필요합니다.")
                        } else {
                            format!("Server {slot}: url is required for network transports.")
                        });
                    } else if !server.url.starts_with("http://")
                        && !server.url.starts_with("https://")
                    {
                        messages.push(if self.state.locale == AppLocale::Korean {
                            format!("서버 {slot}: URL은 http:// 또는 https:// 로 시작해야 합니다.")
                        } else {
                            format!("Server {slot}: url must start with http:// or https://.")
                        });
                    }
                }
            }

            for key in server.env.keys() {
                if key.trim().is_empty() {
                    messages.push(if self.state.locale == AppLocale::Korean {
                        format!("서버 {slot}: 환경 변수 키가 비어 있습니다.")
                    } else {
                        format!("Server {slot}: env key cannot be empty.")
                    });
                }
            }
        }

        messages
    }

    fn copy_generated_json(&mut self, ctx: &Context) {
        let json = self.generated_json();
        ctx.copy_text(json);
        self.status = self.txt(UiText::StatusCopiedJson).to_owned();
    }

    fn copy_instructions(&mut self, ctx: &Context) {
        let profile = self.selected_profile();
        let text = if self.state.locale == AppLocale::Korean {
            format!(
                "1. OpenCode 설정으로 이동하세요.\n2. {} 프로필의 MCP 설정 영역을 찾습니다.\n3. 생성한 JSON을 붙여넣거나 병합합니다.\n4. stdio 사용 서버는 PATH에 실행 바이너리가 있는지 확인하세요.\n5. 저장 후 OpenCode를 다시 시작/리로드하세요.\n\n대상 경로:\n{}\n\n메모:\n{}",
                profile.scope.label(self.state.locale),
                profile.open_code_path,
                profile.target_hint
            )
        } else {
            format!(
                "1. Open OpenCode settings.\n2. Locate the MCP configuration area for the {} profile.\n3. Paste the generated JSON into that section or merge the mcpServers block.\n4. If your MCP server uses stdio, confirm the command is available on PATH.\n5. Restart or reload OpenCode after saving.\n\nTarget path hint:\n{}\n\nNotes:\n{}",
                profile.scope.label(self.state.locale),
                profile.open_code_path,
                profile.target_hint
            )
        };
        ctx.copy_text(text);
        self.status = self.txt(UiText::StatusCopiedSetup).to_owned();
    }
}

impl eframe::App for OpenMcpApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut copy_json = false;
        let mut copy_setup = false;
        let mut load_config = false;
        let mut install_enabled = false;
        let mut uninstall_enabled = false;
        let mut selected_profile = None;

        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading(self.txt(UiText::AppTitle));
                ui.label(self.txt(UiText::AppSubtitle));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(self.txt(UiText::BtnSave)).clicked() {
                        self.save();
                    }
                    if ui.button(self.txt(UiText::BtnUninstallEnabled)).clicked() {
                        uninstall_enabled = true;
                    }
                    if ui.button(self.txt(UiText::BtnInstallEnabled)).clicked() {
                        install_enabled = true;
                    }
                    if ui.button(self.txt(UiText::BtnLoadConfig)).clicked() {
                        load_config = true;
                    }
                    if ui.button(self.txt(UiText::BtnCopySetup)).clicked() {
                        copy_setup = true;
                    }
                    if ui.button(self.txt(UiText::BtnCopyJson)).clicked() {
                        copy_json = true;
                    }
                });
            });
        });

        SidePanel::left("profiles")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading(self.txt(UiText::Profiles));
                ui.label(self.txt(UiText::ProfilesHint));
                ui.add_space(8.0);

                ScrollArea::vertical().show(ui, |ui| {
                    for index in 0..self.state.profiles.len() {
                        let selected = index == self.state.selected_profile;
                        let label = {
                            let profile = &self.state.profiles[index];
                            format!(
                                "{}  {}",
                                profile.scope.label(self.state.locale),
                                profile.name
                            )
                        };
                        if ui.selectable_label(selected, label).clicked() {
                            selected_profile = Some(index);
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button(self.txt(UiText::BtnNew)).clicked() {
                        self.add_profile();
                    }
                    if ui.button(self.txt(UiText::BtnDuplicate)).clicked() {
                        self.duplicate_profile();
                    }
                    if ui
                        .add_enabled(
                            self.state.profiles.len() > 1,
                            Button::new(self.txt(UiText::BtnDelete)),
                        )
                        .clicked()
                    {
                        self.delete_selected_profile();
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                self.render_editor(&mut columns[0]);
                self.render_preview(&mut columns[1], ctx);
            });
        });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let dirty = if self.dirty {
                    self.txt(UiText::StatusUnsaved)
                } else {
                    self.txt(UiText::StatusSaved)
                };
                ui.label(format!("{dirty} | {}", self.status));
            });
        });

        if let Some(index) = selected_profile {
            self.state.selected_profile = index;
            self.refresh_managed_servers();
        }
        if copy_json {
            self.copy_generated_json(ctx);
        }
        if copy_setup {
            self.copy_instructions(ctx);
        }
        if install_enabled {
            self.install_enabled_servers();
        }
        if uninstall_enabled {
            self.uninstall_enabled_servers();
        }
        if load_config {
            self.refresh_managed_servers();
        }
    }
}

impl OpenMcpApp {
    fn render_editor(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let locale = self.state.locale;
            let profile_name_hint = self.txt(UiText::ProfileNameHint);
            let scope_label = self.txt(UiText::ScopeLabel);
            let scope_hint = {
                let profile = self.selected_profile();
                profile.scope.hint(locale).to_owned()
            };
            let open_code_path_label = self.txt(UiText::OpenCodePathLabel);
            let open_code_path_hint = self.txt(UiText::OpenCodePathHint);
            let operator_note_label = self.txt(UiText::OperatorNoteLabel);
            let operator_note_hint = self.txt(UiText::OperatorNoteHint);
            let mcp_servers_section = self.txt(UiText::McpServersSection);

            ui.heading(self.txt(UiText::ProfileSection));
            {
                let profile = self.selected_profile_mut();
                if ui
                    .add(TextEdit::singleline(&mut profile.name).hint_text(profile_name_hint))
                    .changed()
                {
                    self.dirty = true;
                }
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(scope_label);
                for scope in [ProfileScope::Project, ProfileScope::Global] {
                    let changed = {
                        let profile = self.selected_profile_mut();
                        let scope_text = scope.label(locale);
                        ui.selectable_value(&mut profile.scope, scope, scope_text)
                            .changed()
                    };
                    if changed {
                        self.dirty = true;
                    }
                }
            });
            ui.label(RichText::new(scope_hint).small().color(Color32::GRAY));

            ui.add_space(8.0);
            ui.label(open_code_path_label);
            {
                let profile = self.selected_profile_mut();
                if ui
                    .add(
                        TextEdit::singleline(&mut profile.open_code_path)
                            .hint_text(open_code_path_hint),
                    )
                    .changed()
                {
                    self.dirty = true;
                }
            }

            ui.label(operator_note_label);
            {
                let profile = self.selected_profile_mut();
                if ui
                    .add(
                        TextEdit::multiline(&mut profile.target_hint)
                            .desired_rows(2)
                            .hint_text(operator_note_hint),
                    )
                    .changed()
                {
                    self.dirty = true;
                }
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(10.0);
            ui.heading(mcp_servers_section);

            let managed_ids: BTreeSet<String> = self.managed_server_ids.iter().cloned().collect();
            let mut remove_index = None;
            let mut queued_action = None;
            let mut mark_dirty = false;
            let server_count = self.selected_profile().servers.len();
            for index in 0..server_count {
                let is_installed = self
                    .selected_profile()
                    .servers
                    .get(index)
                    .map(|server| managed_ids.contains(&server.id))
                    .unwrap_or(false);
                ui.group(|ui| {
                    let action = {
                        let profile = self.selected_profile_mut();
                        render_server_card(
                            ui,
                            &mut profile.servers[index],
                            index,
                            &mut mark_dirty,
                            is_installed,
                            locale,
                        )
                    };
                    if action.is_some() {
                        queued_action = action;
                    }
                    if ui
                        .add_enabled(
                            server_count > 1,
                            Button::new(self.txt(UiText::RemoveServerButton)),
                        )
                        .clicked()
                    {
                        remove_index = Some(index);
                    }
                });
                ui.add_space(10.0);
            }
            if mark_dirty {
                self.dirty = true;
            }

            if let Some(index) = remove_index {
                let profile = self.selected_profile_mut();
                profile.servers.remove(index);
                self.dirty = true;
                self.status = self.txt(UiText::StatusServerRemoved).to_owned();
            }

            if let Some(action) = queued_action {
                self.apply_single_server_action(action);
            }

            if ui.button(self.txt(UiText::AddServerButton)).clicked() {
                let next_id = self.selected_profile().servers.len() + 1;
                let profile = self.selected_profile_mut();
                profile.servers.push(McpServer {
                    id: format!("server_{next_id}"),
                    ..McpServer::default()
                });
                self.dirty = true;
                self.status = self.txt(UiText::StatusServerAdded).to_owned();
            }
        });
    }

    fn render_preview(&mut self, ui: &mut Ui, ctx: &Context) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading(self.txt(UiText::GeneratedConfigTitle));
            ui.label(self.txt(UiText::GeneratedConfigHint));
            let profile = self.selected_profile().clone();
            let managed_path = profile.open_code_path.clone();

            let errors = self.validation_messages();
            if errors.is_empty() {
                ui.colored_label(
                    Color32::from_rgb(70, 145, 80),
                    self.txt(UiText::ValidationPassed),
                );
            } else {
                ui.colored_label(
                    Color32::from_rgb(190, 120, 60),
                    self.txt(UiText::ValidationFixHint),
                );
                for message in errors {
                    ui.label(format!("- {message}"));
                }
            }

            ui.add_space(10.0);
            let mut generated = self.generated_json();
            ui.add(
                TextEdit::multiline(&mut generated)
                    .code_editor()
                    .desired_rows(24)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            ui.heading(self.txt(UiText::OpenCodeGuideTitle));
            ui.label(format!(
                "{} {}",
                self.txt(UiText::FlowScopePrefix),
                profile.scope.label(self.state.locale)
            ));
            ui.label(format!(
                "{}: {}",
                self.txt(UiText::ManagedPathLabel),
                profile.open_code_path
            ));
            ui.label(profile.target_hint.as_str());

            ui.add_space(8.0);
            ui.label(self.txt(UiText::FlowTitle));
            ui.label(self.txt(UiText::FlowStep1));
            ui.label(self.txt(UiText::FlowStep2));
            ui.label(self.txt(UiText::FlowStep3));
            ui.label(self.txt(UiText::FlowStep4));

            ui.add_space(10.0);
            if ui.button(self.txt(UiText::CopyJsonAgain)).clicked() {
                self.copy_generated_json(ctx);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.heading(self.txt(UiText::ConfigManagerTitle));
            let mut reload_clicked = false;
            if ui
                .button(self.txt(UiText::ReloadInstalledStateButton))
                .clicked()
            {
                reload_clicked = true;
            }
            if reload_clicked {
                self.refresh_managed_servers();
            }
            ui.label(format!(
                "{}: {}",
                self.txt(UiText::ManagedPathLabel),
                managed_path
            ));
            if self.managed_server_ids.is_empty() {
                ui.colored_label(
                    Color32::from_rgb(130, 130, 130),
                    self.txt(UiText::NoServerInConfig),
                );
            } else {
                ui.colored_label(
                    Color32::from_rgb(70, 145, 80),
                    self.txt(UiText::InstalledServerIds),
                );
                for id in &self.managed_server_ids {
                    ui.label(format!("- {id}"));
                }
            }
        });
    }
}

fn render_server_card(
    ui: &mut Ui,
    server: &mut McpServer,
    index: usize,
    dirty: &mut bool,
    installed: bool,
    locale: AppLocale,
) -> Option<ServerConfigAction> {
    let mut requested = None;

    let label = if locale == AppLocale::Korean {
        "서버"
    } else {
        "Server"
    };

    ui.horizontal(|ui| {
        ui.heading(format!("{label} {}", index + 1));
        if ui
            .checkbox(
                &mut server.enabled,
                if locale == AppLocale::Korean {
                    "활성"
                } else {
                    "Enabled"
                },
            )
            .changed()
        {
            *dirty = true;
        }
        if installed {
            ui.colored_label(
                Color32::from_rgb(70, 145, 80),
                locale.t(UiText::ServerCardInstalled),
            );
        } else {
            ui.colored_label(
                Color32::from_rgb(130, 130, 130),
                locale.t(UiText::ServerCardNotInstalled),
            );
        }
    });

    ui.label(locale.t(UiText::ServerIdLabel));
    if ui
        .add(TextEdit::singleline(&mut server.id).hint_text("filesystem"))
        .changed()
    {
        *dirty = true;
    }

    ui.horizontal(|ui| {
        ui.label(locale.t(UiText::TransportLabel));
        for transport in [
            TransportKind::Stdio,
            TransportKind::Sse,
            TransportKind::Http,
        ] {
            if ui
                .selectable_value(&mut server.transport, transport, transport.label(locale))
                .changed()
            {
                *dirty = true;
            }
        }
    });

    match server.transport {
        TransportKind::Stdio => {
            ui.label(locale.t(UiText::CommandLabel));
            if ui
                .add(TextEdit::singleline(&mut server.command).hint_text("npx"))
                .changed()
            {
                *dirty = true;
            }

            ui.label(locale.t(UiText::ArgsLabel));
            let mut args_text = server.args.join(" ");
            if ui
                .add(
                    TextEdit::multiline(&mut args_text)
                        .desired_rows(2)
                        .hint_text("-y @modelcontextprotocol/server-filesystem ."),
                )
                .changed()
            {
                server.args = split_shellish(&args_text);
                *dirty = true;
            }

            ui.label(locale.t(UiText::WorkingDirectoryLabel));
            if ui
                .add(TextEdit::singleline(&mut server.cwd).hint_text("."))
                .changed()
            {
                *dirty = true;
            }
        }
        TransportKind::Sse | TransportKind::Http => {
            ui.label(locale.t(UiText::EndpointLabel));
            if ui
                .add(TextEdit::singleline(&mut server.url).hint_text("http://localhost:3000/mcp"))
                .changed()
            {
                *dirty = true;
            }
        }
    }

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if installed
            && ui
                .button(locale.t(UiText::BtnUninstallServerCard))
                .clicked()
        {
            requested = Some(ServerConfigAction::Uninstall(index));
        }
        if !installed && ui.button(locale.t(UiText::BtnInstallServerCard)).clicked() {
            requested = Some(ServerConfigAction::Install(index));
        }
    });
    ui.add_space(4.0);

    ui.label(locale.t(UiText::EnvVarsLabel));
    let mut remove_key = None;
    let keys: Vec<String> = server.env.keys().cloned().collect();
    for key in keys {
        let mut current_key = key.clone();
        let mut current_value = server.env.get(&key).cloned().unwrap_or_default();

        ui.horizontal(|ui| {
            if ui
                .add(TextEdit::singleline(&mut current_key).desired_width(180.0))
                .changed()
            {
                *dirty = true;
            }
            if ui
                .add(TextEdit::singleline(&mut current_value).desired_width(240.0))
                .changed()
            {
                *dirty = true;
            }
            if ui.button("x").clicked() {
                remove_key = Some(key.clone());
            }
        });

        if current_key != key {
            server.env.remove(&key);
            server.env.insert(current_key, current_value);
        } else {
            server.env.insert(key, current_value);
        }
    }

    if let Some(key) = remove_key {
        server.env.remove(&key);
        *dirty = true;
    }

    if ui.button(locale.t(UiText::AddEnvVarButton)).clicked() {
        let key = format!("NEW_ENV_{}", server.env.len() + 1);
        server.env.insert(key, String::new());
        *dirty = true;
    }

    ui.add_space(6.0);
    ui.label(locale.t(UiText::NotesLabel));
    if ui
        .add(
            TextEdit::multiline(&mut server.notes)
                .desired_rows(3)
                .hint_text(locale.t(UiText::NotesHint)),
        )
        .changed()
    {
        *dirty = true;
    }

    requested
}

fn split_shellish(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
