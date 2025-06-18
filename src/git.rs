use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

pub struct Git;

impl Git {
    /// Git 저장소인지 확인
    pub async fn is_git_repo() -> Result<bool> {
        let output = Command::new("git")
            .args(&["rev-parse", "--git-dir"])
            .output()
            .await?;

        Ok(output.status.success())
    }

    /// Git 저장소의 루트 경로 찾기
    pub async fn get_git_root() -> Result<PathBuf> {
        let output = Command::new("git")
            .args(&["rev-parse", "--show-toplevel"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Not in a git repository"));
        }

        let path_str = String::from_utf8(output.stdout)?.trim().to_string();

        Ok(PathBuf::from(path_str))
    }

    /// Git hooks 디렉토리 경로 가져오기
    pub async fn get_hooks_dir() -> Result<PathBuf> {
        let git_root = Self::get_git_root().await?;
        Ok(git_root.join(".git").join("hooks"))
    }

    /// Git hooks 디렉토리 설정
    pub async fn setup_hooks_dir() -> Result<()> {
        let hooks_dir = Self::get_hooks_dir().await?;

        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir).await?;
        }

        Ok(())
    }

    /// 특정 hook 파일의 경로 가져오기
    pub async fn get_hook_path(hook_name: &str) -> Result<PathBuf> {
        let hooks_dir = Self::get_hooks_dir().await?;
        Ok(hooks_dir.join(hook_name))
    }

    /// Hook 파일이 존재하는지 확인
    #[allow(dead_code)]
    pub async fn hook_exists(hook_name: &str) -> Result<bool> {
        let hook_path = Self::get_hook_path(hook_name).await?;
        Ok(hook_path.exists())
    }

    /// Git 설정 값 가져오기
    #[allow(dead_code)]
    pub async fn get_config(key: &str) -> Result<Option<String>> {
        let output = Command::new("git")
            .args(&["config", "--get", key])
            .output()
            .await?;

        if output.status.success() {
            let value = String::from_utf8(output.stdout)?.trim().to_string();
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Git 설정 값 설정하기
    #[allow(dead_code)]
    pub async fn set_config(key: &str, value: &str) -> Result<()> {
        let output = Command::new("git")
            .args(&["config", key, value])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to set git config: {}", key));
        }

        Ok(())
    }
}
