# System Patterns: rusky

## 아키텍처 개요

### 전체 시스템 구조
```
rusky/
├── CLI Layer (src/main.rs, src/cli.rs)
├── Core Logic Layer (src/config.rs, src/git.rs, src/hooks.rs)
├── npm Integration Layer (package.json, install.js, uninstall.js)
└── Distribution Layer (Cargo.toml, README.md, LICENSE)
```

### 핵심 컴포넌트

#### 1. CLI Layer
- **역할**: 사용자 인터페이스 제공
- **기술**: clap 라이브러리 사용
- **패턴**: Command Pattern 적용
- **파일**: `src/main.rs`, `src/cli.rs`

#### 2. Configuration Management
- **역할**: 설정 파일 관리 (.rusky/config.json)
- **기술**: serde_json 사용
- **패턴**: Repository Pattern
- **파일**: `src/config.rs`

#### 3. Git Integration
- **역할**: Git 저장소 및 hooks 디렉토리 관리
- **기술**: tokio::process::Command 사용
- **패턴**: Facade Pattern
- **파일**: `src/git.rs`

#### 4. Hook Management
- **역할**: Git hook 파일 생성/삭제/관리
- **기술**: tokio::fs 사용
- **패턴**: Template Method Pattern
- **파일**: `src/hooks.rs`

## 핵심 기술적 결정

### 1. 비동기 프로그래밍 (Tokio)
**결정**: 모든 I/O 작업에 async/await 사용
**이유**: 
- 파일 시스템 작업 최적화
- Git 명령어 실행 시 블로킹 방지
- 향후 확장성 고려

### 2. 에러 처리 (anyhow)
**결정**: anyhow 크레이트 사용
**이유**:
- 간단한 에러 체인
- 사용자 친화적 에러 메시지
- 개발 생산성 향상

### 3. 설정 관리 (JSON)
**결정**: JSON 형식으로 설정 저장
**이유**:
- 사람이 읽기 쉬움
- 널리 지원되는 형식
- Node.js 생태계와 호환성

### 4. CLI 인터페이스 (clap)
**결정**: clap derive 매크로 사용
**이유**:
- 타입 안전성
- 자동 도움말 생성
- 확장성

## 디자인 패턴

### 1. Command Pattern
```rust
enum Commands {
    Init,
    Add { hook: String, command: String },
    Remove { hook: String },
    List,
    Install,
    Uninstall,
}
```

### 2. Repository Pattern
```rust
impl Config {
    pub async fn load() -> Result<Self>
    pub async fn save(&self) -> Result<()>
    pub fn add_hook(&mut self, hook_name: String, command: String)
    pub fn remove_hook(&mut self, hook_name: &str) -> bool
}
```

### 3. Facade Pattern
```rust
impl Git {
    pub async fn is_git_repo() -> Result<bool>
    pub async fn get_git_root() -> Result<PathBuf>
    pub async fn setup_hooks_dir() -> Result<()>
}
```

## 데이터 플로우

### Hook 추가 플로우
```
User Input → CLI Parser → Config Load → Hook Validation → 
Config Update → Hook File Creation → Config Save → Success Message
```

### Hook 실행 플로우
```
Git Event → Hook Script → rusky Message → User Command → 
Command Execution → Result Display → Exit Code
```

## 보안 고려사항

### 1. 파일 권한
- Hook 파일에 실행 권한 (755) 설정
- 설정 파일 읽기/쓰기 권한만 부여

### 2. 명령어 실행
- 사용자 입력 명령어 직접 실행 (신뢰 가정)
- 향후 샌드박스 환경 고려

### 3. 경로 검증
- Git 저장소 존재 여부 확인
- 상대 경로 사용으로 보안 강화

## 성능 최적화

### 1. 컴파일 최적화
- Release 빌드 시 최적화 레벨 3
- LTO (Link Time Optimization) 적용 고려

### 2. 메모리 최적화
- 스택 할당 우선 사용
- 불필요한 클론 최소화

### 3. I/O 최적화
- 비동기 I/O 사용
- 배치 처리 고려

## 확장성 고려사항

### 1. 플러그인 시스템
- 향후 사용자 정의 플러그인 지원
- Hook 템플릿 시스템

### 2. 설정 확장
- YAML/TOML 지원 고려
- 환경별 설정 분리

### 3. 다국어 지원
- i18n 라이브러리 통합 고려
- 메시지 외부화 