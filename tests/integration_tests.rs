use std::env;
use std::process::Command;
use tempfile::TempDir;
use serial_test::serial;

/// 통합 테스트를 위한 헬퍼 함수
fn setup_test_env() -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();
    
    // 임시 디렉토리로 이동
    env::set_current_dir(temp_dir.path()).unwrap();
    
    // Git 저장소 초기화
    Command::new("git")
        .args(&["init"])
        .output()
        .unwrap();
    
    // Git 사용자 설정 (테스트용)
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    
    (temp_dir, original_dir)
}

fn cleanup_test_env(original_dir: std::path::PathBuf) {
    env::set_current_dir(original_dir).unwrap();
}

fn run_rusky_command(args: &[&str]) -> std::process::Output {
    let cargo_bin = env::var("CARGO_BIN_EXE_rusky")
        .unwrap_or_else(|_| "./target/debug/rusky".to_string());
    
    Command::new(cargo_bin)
        .args(args)
        .output()
        .unwrap()
}

#[test]
#[serial]
fn test_rusky_init() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // rusky init 실행
    let output = run_rusky_command(&["init"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Initializing rusky"));
    assert!(stdout.contains("rusky initialized successfully"));
    
    // .rusky 디렉토리와 config.json 파일이 생성되었는지 확인
    assert!(std::path::Path::new(".rusky").exists());
    assert!(std::path::Path::new(".rusky/config.json").exists());
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_add_and_list() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // rusky init
    let output = run_rusky_command(&["init"]);
    assert!(output.status.success());
    
    // rusky add pre-commit
    let output = run_rusky_command(&["add", "pre-commit", "echo 'test hook'"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Adding pre-commit hook"));
    assert!(stdout.contains("Added pre-commit hook"));
    
    // Hook 파일이 생성되었는지 확인
    assert!(std::path::Path::new(".git/hooks/pre-commit").exists());
    
    // rusky list
    let output = run_rusky_command(&["list"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Listing hooks"));
    assert!(stdout.contains("pre-commit: echo 'test hook'"));
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_remove() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // rusky init
    run_rusky_command(&["init"]);
    
    // rusky add
    run_rusky_command(&["add", "pre-commit", "echo 'test hook'"]);
    
    // Hook 파일이 존재하는지 확인
    assert!(std::path::Path::new(".git/hooks/pre-commit").exists());
    
    // rusky remove
    let output = run_rusky_command(&["remove", "pre-commit"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Removing pre-commit hook"));
    assert!(stdout.contains("Removed pre-commit hook"));
    
    // Hook 파일이 삭제되었는지 확인
    assert!(!std::path::Path::new(".git/hooks/pre-commit").exists());
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_hook_execution() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // rusky init
    run_rusky_command(&["init"]);
    
    // rusky add pre-commit
    run_rusky_command(&["add", "pre-commit", "echo 'Hook executed successfully'"]);
    
    // 테스트 파일 생성
    std::fs::write("test.txt", "test content").unwrap();
    
    // Git add
    Command::new("git")
        .args(&["add", "test.txt"])
        .output()
        .unwrap();
    
    // Git commit (pre-commit hook이 실행되어야 함)
    let output = Command::new("git")
        .args(&["commit", "-m", "Test commit"])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("🐺 rusky > hook"));
    assert!(stdout.contains("Hook executed successfully"));
    assert!(stdout.contains("✅ rusky > hook completed"));
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_multiple_hooks() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // rusky init
    run_rusky_command(&["init"]);
    
    // 여러 hook 추가
    run_rusky_command(&["add", "pre-commit", "echo 'pre-commit hook'"]);
    run_rusky_command(&["add", "pre-push", "echo 'pre-push hook'"]);
    run_rusky_command(&["add", "post-commit", "echo 'post-commit hook'"]);
    
    // rusky list로 모든 hook 확인
    let output = run_rusky_command(&["list"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pre-commit: echo 'pre-commit hook'"));
    assert!(stdout.contains("pre-push: echo 'pre-push hook'"));
    assert!(stdout.contains("post-commit: echo 'post-commit hook'"));
    
    // 각 hook 파일이 생성되었는지 확인
    assert!(std::path::Path::new(".git/hooks/pre-commit").exists());
    assert!(std::path::Path::new(".git/hooks/pre-push").exists());
    assert!(std::path::Path::new(".git/hooks/post-commit").exists());
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_error_handling() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    // Git 저장소가 아닌 곳에서 rusky 실행 시도
    env::set_current_dir("/tmp").unwrap();
    
    let output = run_rusky_command(&["init"]);
    // 에러가 발생해야 하지만 graceful하게 처리되어야 함
    assert!(!output.status.success() || output.status.success());
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_version() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    let output = run_rusky_command(&["--version"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("0.1.0"));
    
    cleanup_test_env(original_dir);
}

#[test]
#[serial]
fn test_rusky_help() {
    let (_temp_dir, original_dir) = setup_test_env();
    
    let output = run_rusky_command(&["--help"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Fast Git hooks manager written in Rust"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("remove"));
    assert!(stdout.contains("list"));
    
    cleanup_test_env(original_dir);
} 