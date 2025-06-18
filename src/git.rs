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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[tokio::test]
    async fn test_is_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동 (Git 저장소가 아닌 상태)
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let is_git = Git::is_git_repo().await.unwrap_or(false);
        assert!(!is_git);
        
        // Git 저장소 초기화
        let output = tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await;
        
        if output.is_ok() && output.unwrap().status.success() {
            let is_git = Git::is_git_repo().await.unwrap();
            assert!(is_git);
        }
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_get_git_root() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동
        env::set_current_dir(temp_dir.path()).unwrap();
        
        // Git 저장소가 아닌 상태에서는 에러가 발생해야 함
        let result = Git::get_git_root().await;
        assert!(result.is_err());
        
        // Git 저장소 초기화
        let output = tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await;
        
        if output.is_ok() && output.unwrap().status.success() {
            let git_root = Git::get_git_root().await.unwrap();
            // 경로 정규화를 위해 canonicalize 사용
            let expected_path = temp_dir.path().canonicalize().unwrap();
            let actual_path = git_root.canonicalize().unwrap();
            assert_eq!(actual_path, expected_path);
        }
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_get_hooks_dir() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동하고 Git 저장소 초기화
        env::set_current_dir(temp_dir.path()).unwrap();
        tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await
            .unwrap();
        
        let hooks_dir = Git::get_hooks_dir().await.unwrap();
        let expected_path = temp_dir.path().join(".git").join("hooks");
        assert_eq!(hooks_dir, expected_path);
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_setup_hooks_dir() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동하고 Git 저장소 초기화
        env::set_current_dir(temp_dir.path()).unwrap();
        tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await
            .unwrap();
        
        // hooks 디렉토리는 이미 git init으로 생성되어 있어야 함
        let hooks_dir = Git::get_hooks_dir().await.unwrap();
        assert!(hooks_dir.exists());
        
        // setup_hooks_dir 호출 (이미 존재하는 디렉토리에 대해서도 성공해야 함)
        let result = Git::setup_hooks_dir().await;
        assert!(result.is_ok());
        assert!(hooks_dir.exists());
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_get_hook_path() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동하고 Git 저장소 초기화
        env::set_current_dir(temp_dir.path()).unwrap();
        tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await
            .unwrap();
        
        let hook_path = Git::get_hook_path("pre-commit").await.unwrap();
        let expected_path = temp_dir.path().join(".git").join("hooks").join("pre-commit");
        assert_eq!(hook_path, expected_path);
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_hook_exists() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동하고 Git 저장소 초기화
        env::set_current_dir(temp_dir.path()).unwrap();
        tokio::process::Command::new("git")
            .args(&["init"])
            .output()
            .await
            .unwrap();
        
        // 존재하지 않는 hook
        let exists = Git::hook_exists("pre-commit").await.unwrap();
        assert!(!exists);
        
        // hook 파일 생성
        let hook_path = Git::get_hook_path("pre-commit").await.unwrap();
        tokio::fs::write(&hook_path, "#!/bin/sh\necho test").await.unwrap();
        
        // 이제 존재해야 함
        let exists = Git::hook_exists("pre-commit").await.unwrap();
        assert!(exists);
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }
}
