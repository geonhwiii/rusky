# 🧪 rusky 테스트 가이드

rusky는 포괄적인 테스트 스위트를 제공하여 코드 품질과 안정성을 보장합니다.

## 📋 테스트 구조

### 단위 테스트 (Unit Tests)
각 모듈별로 개별 함수와 로직을 테스트합니다.

#### `src/config.rs` 테스트
- ✅ `test_config_default` - 기본 설정 생성 테스트
- ✅ `test_add_hook` - Hook 추가 기능 테스트
- ✅ `test_remove_hook` - Hook 제거 기능 테스트
- ✅ `test_get_hook` - Hook 조회 기능 테스트
- ✅ `test_has_hooks` - Hook 존재 여부 확인 테스트
- ✅ `test_save_and_load_config` - 설정 저장/로드 테스트
- ✅ `test_load_nonexistent_config` - 존재하지 않는 설정 로드 테스트

#### `src/cli.rs` 테스트
- ✅ `test_is_valid_hook_name` - Hook 이름 유효성 검증 테스트
- 🔄 `test_init_success` - 초기화 성공 테스트 (Git 의존성)
- 🔄 `test_init_not_git_repo` - Git 저장소가 아닌 경우 테스트
- 🔄 `test_add_hook_success` - Hook 추가 성공 테스트
- 🔄 `test_add_hook_invalid_name` - 잘못된 Hook 이름 테스트
- 🔄 `test_remove_hook_success` - Hook 제거 성공 테스트
- 🔄 `test_remove_hook_not_found` - 존재하지 않는 Hook 제거 테스트
- 🔄 `test_list_hooks_empty` - 빈 Hook 목록 테스트
- 🔄 `test_list_hooks_with_content` - Hook이 있는 목록 테스트
- 🔄 `test_install_hooks` - Hook 설치 테스트
- 🔄 `test_uninstall_hooks` - Hook 제거 테스트

#### `src/hooks.rs` 테스트
- ✅ `test_generate_hook_script` - Hook 스크립트 생성 테스트
- 🔄 `test_create_and_remove_hook_file` - Hook 파일 생성/삭제 테스트
- 🔄 `test_remove_nonexistent_hook_file` - 존재하지 않는 Hook 파일 삭제 테스트
- 🔄 `test_is_rusky_hook` - rusky Hook 여부 확인 테스트

#### `src/git.rs` 테스트
- 🔄 `test_is_git_repo` - Git 저장소 확인 테스트
- 🔄 `test_get_git_root` - Git 루트 디렉토리 조회 테스트
- 🔄 `test_get_hooks_dir` - Git hooks 디렉토리 조회 테스트
- 🔄 `test_setup_hooks_dir` - hooks 디렉토리 설정 테스트
- 🔄 `test_get_hook_path` - Hook 파일 경로 조회 테스트
- 🔄 `test_hook_exists` - Hook 파일 존재 여부 확인 테스트

### 통합 테스트 (Integration Tests)
실제 CLI 명령어를 통한 end-to-end 테스트입니다.

#### `tests/integration_tests.rs`
- 🔄 `test_rusky_init` - rusky 초기화 통합 테스트
- 🔄 `test_rusky_add_and_list` - Hook 추가 및 목록 조회 테스트
- 🔄 `test_rusky_remove` - Hook 제거 통합 테스트
- 🔄 `test_rusky_hook_execution` - 실제 Git commit 시 Hook 실행 테스트
- 🔄 `test_rusky_multiple_hooks` - 여러 Hook 관리 테스트
- 🔄 `test_rusky_error_handling` - 에러 처리 테스트
- 🔄 `test_rusky_version` - 버전 출력 테스트
- 🔄 `test_rusky_help` - 도움말 출력 테스트

## 🚀 테스트 실행 방법

### 모든 테스트 실행
```bash
cargo test
```

### 특정 모듈 테스트 실행
```bash
# Config 모듈 테스트만
cargo test config::tests

# CLI 모듈 테스트만
cargo test cli::tests

# Hooks 모듈 테스트만
cargo test hooks::tests

# Git 모듈 테스트만
cargo test git::tests
```

### 특정 테스트 함수 실행
```bash
# Hook 이름 검증 테스트만
cargo test test_is_valid_hook_name

# Hook 스크립트 생성 테스트만
cargo test test_generate_hook_script

# Config 기본값 테스트만
cargo test test_config_default
```

### 통합 테스트만 실행
```bash
cargo test --test integration_tests
```

### 단위 테스트만 실행 (통합 테스트 제외)
```bash
cargo test --lib
```

### 테스트 출력 보기
```bash
cargo test -- --nocapture
```

### 테스트 병렬 실행 제어
```bash
# 순차 실행 (Git 관련 테스트에서 필요할 수 있음)
cargo test -- --test-threads=1
```

## 🔧 테스트 환경 설정

### 의존성
테스트 실행을 위해 다음 의존성들이 필요합니다:

```toml
[dev-dependencies]
tempfile = "3.0"        # 임시 파일/디렉토리 생성
tokio-test = "0.4"      # 비동기 테스트 헬퍼
serial_test = "3.0"     # 순차 테스트 실행
```

### Git 설정
일부 테스트는 Git이 시스템에 설치되어 있어야 합니다:

```bash
# Git 설치 확인
git --version

# 테스트용 Git 설정 (필요한 경우)
git config --global user.name "Test User"
git config --global user.email "test@example.com"
```

## 📊 테스트 상태

### ✅ 안정적인 테스트 (Git 독립적)
- Config 모듈 모든 테스트
- Hook 스크립트 생성 테스트
- Hook 이름 검증 테스트

### 🔄 환경 의존적 테스트 (Git 필요)
- CLI 모듈의 대부분 테스트
- Git 모듈의 모든 테스트
- Hooks 모듈의 파일 조작 테스트
- 모든 통합 테스트

## 🐛 테스트 문제 해결

### 일반적인 문제

#### Git 명령어를 찾을 수 없음
```bash
# Git 설치 확인
which git

# macOS에서 Command Line Tools 설치
xcode-select --install
```

#### 권한 문제
```bash
# 실행 권한이 필요한 경우
chmod +x target/debug/rusky
```

#### 임시 디렉토리 문제
```bash
# 임시 디렉토리 정리
cargo clean
```

### 테스트 격리
각 테스트는 독립적인 임시 디렉토리에서 실행되어 서로 영향을 주지 않습니다:

```rust
let temp_dir = TempDir::new().unwrap();
let original_dir = env::current_dir().unwrap();
env::set_current_dir(temp_dir.path()).unwrap();
// ... 테스트 실행 ...
env::set_current_dir(original_dir).unwrap();
```

## 🎯 테스트 작성 가이드

### 새로운 테스트 추가 시 고려사항

1. **독립성**: 각 테스트는 다른 테스트에 영향을 주지 않아야 합니다
2. **정리**: 테스트 후 생성된 파일이나 디렉토리를 정리해야 합니다
3. **에러 처리**: 예상되는 에러 상황도 테스트해야 합니다
4. **문서화**: 테스트의 목적과 기대 결과를 명확히 해야 합니다

### 테스트 네이밍 규칙
```rust
#[tokio::test]
async fn test_[기능]_[상황]() {
    // 예: test_add_hook_success
    // 예: test_init_not_git_repo
}
```

## 🔮 향후 계획

- [ ] CI/CD 파이프라인에서 자동 테스트 실행
- [ ] 코드 커버리지 측정 및 리포팅
- [ ] 성능 벤치마크 테스트 추가
- [ ] Windows 환경 테스트 지원 강화
- [ ] 더 많은 에지 케이스 테스트 추가

---

테스트에 대한 질문이나 개선 제안이 있으시면 이슈를 생성해 주세요! 🚀 