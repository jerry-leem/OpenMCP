# OpenMCP

[![CI](https://github.com/jerryan/OpenMCP/actions/workflows/ci.yml/badge.svg)](https://github.com/jerryan/OpenMCP/actions/workflows/ci.yml)

## 한국어

OpenMCP는 MCP 서버 설정을 폼 기반으로 정리하고, OpenCode에 붙여 넣을 JSON을 생성하거나 직접 설치할 수 있게 도와주는 가벼운 데스크톱 도구입니다.

### 기술 선택

- `Rust + eframe/egui` 기반 네이티브 데스크톱 앱
- 빠른 시작 속도와 낮은 런타임 오버헤드
- 별도 Node 또는 브라우저 런타임 없이 배포 가능
- macOS, Linux, Windows를 하나의 코드베이스로 지원

### 현재 범위

- MCP 서버 정의를 위한 가이드형 폼
- 프로젝트 프로필과 전역 프로필 분리
- 서버 ID, 명령, 인자, 환경 변수, 네트워크 URL 검증
- OpenCode용 JSON 미리보기와 복사 기능
- 설치 안내와 대상 경로 힌트 제공
- OpenCode 설정 파일에 대한 직접 설치, 제거, 새로고침
- 사용자 설정 디렉터리 기준 로컬 자동 저장

### 보안 강화 내용

- OpenCode 설정 파일 쓰기를 임시 파일 기반의 원자적 교체로 변경
- Unix 계열에서 상태 파일과 설정 파일 권한을 `0600`으로 강제
- Unix 계열에서 앱 설정 디렉터리 권한을 `0700`으로 강제
- 관리 대상 경로에 심볼릭 링크가 포함된 경우 읽기/쓰기를 거부
- 네트워크 전송 URL은 `http://` 또는 `https://`만 허용

### 실행

```bash
cargo run
```

### 빌드

```bash
cargo build --release
```

### 테스트

```bash
./scripts/smoke.sh
```

추가 옵션:

- `OPENMCP_SMOKE_RUN_GUI=1` 을 설정하면 5초 GUI 기동 스모크 테스트도 함께 실행합니다.

### 패키징 방향

- macOS: 추후 서명된 `.app`
- Windows: 현재 `.exe`, 이후 설치 프로그램
- Linux: 현재 단일 바이너리, 이후 AppImage 또는 `.deb`

### 참고

이 버전은 MCP 서버 연결 과정을 이해 가능하고 반복 가능하게 만드는 데 초점을 둡니다. 자동 병합 과정에서 위험한 가정을 하지 않도록, 생성된 설정과 설치 동작은 명시적인 OpenCode 설정 경로를 기준으로만 처리합니다.

## English

OpenMCP is a lightweight desktop companion for configuring MCP servers, generating the JSON needed by OpenCode, and installing those entries directly into the target config file.

### Why this stack

- Native desktop app with `Rust + eframe/egui`
- Fast startup and low runtime overhead
- No Node or browser runtime required for the shipped app
- Cross-platform target for macOS, Linux, and Windows from one codebase

### Current scope

- Guided form for defining MCP servers
- Project profile and global profile separation
- Validation for server id, command, args, environment variables, and network URLs
- Generated OpenCode-ready JSON preview and copy flow
- Copyable setup steps and target path guidance
- Direct OpenCode config management: install, uninstall, and refresh installed MCP servers
- Local autosave under the user's config directory

### Security hardening

- OpenCode config writes now use atomic temp-file replacement
- State and config files are forced to `0600` on Unix-like systems
- The app config directory is forced to `0700` on Unix-like systems
- Reads and writes are rejected when a managed path contains symbolic links
- Network transports accept only `http://` and `https://` endpoints

### Run

```bash
cargo run
```

### Build

```bash
cargo build --release
```

### Test

```bash
./scripts/smoke.sh
```

Extra option:

- Set `OPENMCP_SMOKE_RUN_GUI=1` to include a 5-second GUI startup smoke check.

### Packaging direction

- macOS: signed `.app` bundle later
- Windows: `.exe` now, installer later
- Linux: single binary now, AppImage or `.deb` later

### Notes

This version focuses on making MCP wiring understandable and repeatable. To avoid unsafe assumptions during automatic installation, generated config and file management are applied only to an explicit OpenCode config path chosen by the operator.
