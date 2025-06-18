use anyhow::{Result, anyhow};
use colored::*;
use std::path::Path;
use tokio::fs;

use crate::config::Config;
use crate::git::Git;
use crate::hooks::HookManager;

pub async fn init() -> Result<()> {
    // Git 저장소인지 확인
    if !Git::is_git_repo().await? {
        return Err(anyhow!("Not a git repository. Please run 'git init' first."));
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
    println!("{}", "You can now add hooks with: rusky add <hook-name> <command>".dimmed());

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
    
    println!("{}", format!("✅ Added {} hook: {}", hook_name, command).green());
    
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
        println!("  {} {}: {}", "•".blue(), hook_name.bold(), command.dimmed());
    }
    
    Ok(())
}

pub async fn install_hooks() -> Result<()> {
    let config = Config::load().await?;
    
    for (hook_name, command) in &config.hooks {
        HookManager::create_hook_file(hook_name, command).await?;
    }
    
    println!("{}", format!("✅ Installed {} hooks", config.hooks.len()).green());
    
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
        "applypatch-msg", "pre-applypatch", "post-applypatch",
        "pre-commit", "prepare-commit-msg", "commit-msg", "post-commit",
        "pre-rebase", "post-checkout", "post-merge", "pre-push",
        "pre-receive", "update", "post-receive", "post-update",
        "push-to-checkout", "pre-auto-gc", "post-rewrite"
    ];
    
    VALID_HOOKS.contains(&hook_name)
} 