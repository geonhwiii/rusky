use anyhow::{anyhow, Result};
use colored::*;
use std::path::Path;
use tokio::fs;

use crate::config::Config;
use crate::git::Git;
use crate::hooks::HookManager;

pub async fn init() -> Result<()> {
    // Git 저장소인지 확인
    if !Git::is_git_repo().await? {
        return Err(anyhow!(
            "Not a git repository. Please run 'git init' first."
        ));
    }

    // .rusky 디렉토리 생성
    let rusky_dir = Path::new(".rusky");
    if !rusky_dir.exists() {
        fs::create_dir(rusky_dir).await?;
        println!("{}", "✅ Created .rusky directory".green());
    }

    // 기본 설정 파일 생성
    let config = Config::default();
    config.save().await?;
    println!("{}", "✅ Created rusky configuration".green());

    // Git hooks 디렉토리 준비
    Git::setup_hooks_dir().await?;
    println!("{}", "✅ Set up git hooks directory".green());

    println!("{}", "\n🎉 rusky initialized successfully!".bold().green());
    println!(
        "{}",
        "You can now add hooks with: rusky add <hook-name> <command>".dimmed()
    );

    Ok(())
}

pub async fn add_hook(hook_name: &str, command: &str) -> Result<()> {
    let mut config = Config::load().await?;

    // 유효한 hook 이름인지 확인
    if !is_valid_hook_name(hook_name) {
        return Err(anyhow!("Invalid hook name: {}", hook_name));
    }

    config.add_hook(hook_name.to_string(), command.to_string());
    config.save().await?;

    // Hook 파일 생성
    HookManager::create_hook_file(hook_name, command).await?;

    println!(
        "{}",
        format!("✅ Added {} hook: {}", hook_name, command).green()
    );

    Ok(())
}

pub async fn remove_hook(hook_name: &str) -> Result<()> {
    let mut config = Config::load().await?;

    if config.remove_hook(hook_name) {
        config.save().await?;
        HookManager::remove_hook_file(hook_name).await?;
        println!("{}", format!("✅ Removed {} hook", hook_name).green());
    } else {
        println!("{}", format!("⚠️  Hook {} not found", hook_name).yellow());
    }

    Ok(())
}

pub async fn list_hooks() -> Result<()> {
    let config = Config::load().await?;

    if config.hooks.is_empty() {
        println!("{}", "No hooks configured".dimmed());
        return Ok(());
    }

    println!("{}", "Configured hooks:".bold());
    for (hook_name, command) in &config.hooks {
        println!(
            "  {} {}: {}",
            "•".blue(),
            hook_name.bold(),
            command.dimmed()
        );
    }

    Ok(())
}

pub async fn install_hooks() -> Result<()> {
    let config = Config::load().await?;

    for (hook_name, command) in &config.hooks {
        HookManager::create_hook_file(hook_name, command).await?;
    }

    println!(
        "{}",
        format!("✅ Installed {} hooks", config.hooks.len()).green()
    );

    Ok(())
}

pub async fn uninstall_hooks() -> Result<()> {
    let config = Config::load().await?;

    for hook_name in config.hooks.keys() {
        HookManager::remove_hook_file(hook_name).await?;
    }

    println!("{}", "✅ Uninstalled all hooks".green());

    Ok(())
}

fn is_valid_hook_name(hook_name: &str) -> bool {
    const VALID_HOOKS: &[&str] = &[
        "applypatch-msg",
        "pre-applypatch",
        "post-applypatch",
        "pre-commit",
        "prepare-commit-msg",
        "commit-msg",
        "post-commit",
        "pre-rebase",
        "post-checkout",
        "post-merge",
        "pre-push",
        "pre-receive",
        "update",
        "post-receive",
        "post-update",
        "push-to-checkout",
        "pre-auto-gc",
        "post-rewrite",
    ];

    VALID_HOOKS.contains(&hook_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    async fn setup_test_git_repo() -> (TempDir, std::path::PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동
        env::set_current_dir(temp_dir.path()).unwrap();
        
        // Git 저장소 초기화
        tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await
            .unwrap();
        
        (temp_dir, original_dir)
    }

    fn cleanup_test_env(original_dir: std::path::PathBuf) {
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_is_valid_hook_name() {
        assert!(is_valid_hook_name("pre-commit"));
        assert!(is_valid_hook_name("pre-push"));
        assert!(is_valid_hook_name("post-commit"));
        assert!(is_valid_hook_name("commit-msg"));
        
        assert!(!is_valid_hook_name("invalid-hook"));
        assert!(!is_valid_hook_name("random-name"));
        assert!(!is_valid_hook_name(""));
    }

    #[tokio::test]
    async fn test_init_success() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        let result = init().await;
        assert!(result.is_ok());
        
        // .rusky 디렉토리가 생성되었는지 확인
        assert!(Path::new(".rusky").exists());
        assert!(Path::new(".rusky/config.json").exists());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_init_not_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // Git 저장소가 아닌 임시 디렉토리로 이동
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = init().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not a git repository"));
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_add_hook_success() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        // init 먼저 실행
        init().await.unwrap();
        
        let result = add_hook("pre-commit", "echo 'test'").await;
        assert!(result.is_ok());
        
        // 설정 파일에 hook이 추가되었는지 확인
        let config = Config::load().await.unwrap();
        assert_eq!(config.hooks.get("pre-commit"), Some(&"echo 'test'".to_string()));
        
        // Hook 파일이 생성되었는지 확인
        let hook_path = Git::get_hook_path("pre-commit").await.unwrap();
        assert!(hook_path.exists());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_add_hook_invalid_name() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        
        let result = add_hook("invalid-hook", "echo 'test'").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid hook name"));
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_remove_hook_success() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        add_hook("pre-commit", "echo 'test'").await.unwrap();
        
        let result = remove_hook("pre-commit").await;
        assert!(result.is_ok());
        
        // 설정 파일에서 hook이 제거되었는지 확인
        let config = Config::load().await.unwrap();
        assert!(!config.hooks.contains_key("pre-commit"));
        
        // Hook 파일이 삭제되었는지 확인
        let hook_path = Git::get_hook_path("pre-commit").await.unwrap();
        assert!(!hook_path.exists());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_remove_hook_not_found() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        
        let result = remove_hook("non-existent").await;
        assert!(result.is_ok()); // 에러가 발생하지 않아야 함
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_list_hooks_empty() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        
        let result = list_hooks().await;
        assert!(result.is_ok());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_list_hooks_with_content() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        add_hook("pre-commit", "echo 'pre-commit'").await.unwrap();
        add_hook("pre-push", "echo 'pre-push'").await.unwrap();
        
        let result = list_hooks().await;
        assert!(result.is_ok());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_install_hooks() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        
        // 설정에 hook 추가 (파일 생성 없이)
        let mut config = Config::load().await.unwrap();
        config.add_hook("pre-commit".to_string(), "echo 'test'".to_string());
        config.save().await.unwrap();
        
        let result = install_hooks().await;
        assert!(result.is_ok());
        
        // Hook 파일이 생성되었는지 확인
        let hook_path = Git::get_hook_path("pre-commit").await.unwrap();
        assert!(hook_path.exists());
        
        cleanup_test_env(original_dir);
    }

    #[tokio::test]
    async fn test_uninstall_hooks() {
        let (_temp_dir, original_dir) = setup_test_git_repo().await;
        
        init().await.unwrap();
        add_hook("pre-commit", "echo 'test'").await.unwrap();
        add_hook("pre-push", "echo 'test'").await.unwrap();
        
        let result = uninstall_hooks().await;
        assert!(result.is_ok());
        
        // Hook 파일들이 삭제되었는지 확인
        let pre_commit_path = Git::get_hook_path("pre-commit").await.unwrap();
        let pre_push_path = Git::get_hook_path("pre-push").await.unwrap();
        assert!(!pre_commit_path.exists());
        assert!(!pre_push_path.exists());
        
        cleanup_test_env(original_dir);
    }
}
