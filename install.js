#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const BINARY_NAME = 'rusky';
const REPO_URL = 'https://github.com/dan/rusky';

function getPlatform() {
  const platform = os.platform();
  const arch = os.arch();
  
  if (platform === 'darwin') {
    return arch === 'arm64' ? 'darwin-arm64' : 'darwin-x64';
  } else if (platform === 'linux') {
    return arch === 'arm64' ? 'linux-arm64' : 'linux-x64';
  } else if (platform === 'win32') {
    return arch === 'arm64' ? 'win32-arm64' : 'win32-x64';
  }
  
  throw new Error(`Unsupported platform: ${platform}-${arch}`);
}

function downloadBinary() {
  const platform = getPlatform();
  const version = require('./package.json').version;
  const binaryUrl = `${REPO_URL}/releases/download/v${version}/${BINARY_NAME}-${platform}${platform.includes('win32') ? '.exe' : ''}`;
  
  const binDir = path.join(__dirname, 'bin');
  const binaryPath = path.join(binDir, BINARY_NAME + (platform.includes('win32') ? '.exe' : ''));
  
  // bin 디렉토리 생성
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }
  
  console.log(`📦 Downloading rusky binary for ${platform}...`);
  
  try {
    // curl 또는 wget을 사용하여 바이너리 다운로드
    if (process.platform === 'win32') {
      execSync(`powershell -Command "Invoke-WebRequest -Uri '${binaryUrl}' -OutFile '${binaryPath}'"`, { stdio: 'inherit' });
    } else {
      execSync(`curl -L -o "${binaryPath}" "${binaryUrl}"`, { stdio: 'inherit' });
    }
    
    // 실행 권한 부여 (Unix 계열)
    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, '755');
    }
    
    console.log('✅ rusky binary installed successfully!');
    console.log('🚀 Run "rusky init" to get started.');
    
  } catch (error) {
    console.error('❌ Failed to download rusky binary.');
    console.error('Falling back to building from source...');
    buildFromSource();
  }
}

function buildFromSource() {
  console.log('🔨 Building rusky from source...');
  
  try {
    // Rust가 설치되어 있는지 확인
    execSync('cargo --version', { stdio: 'pipe' });
    
    // 릴리스 빌드 실행
    execSync('cargo build --release', { stdio: 'inherit' });
    
    // 빌드된 바이너리를 bin 디렉토리로 복사
    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    
    const sourcePath = path.join(__dirname, 'target', 'release', BINARY_NAME + (process.platform === 'win32' ? '.exe' : ''));
    const targetPath = path.join(binDir, BINARY_NAME + (process.platform === 'win32' ? '.exe' : ''));
    
    fs.copyFileSync(sourcePath, targetPath);
    
    console.log('✅ rusky built and installed successfully!');
    console.log('🚀 Run "rusky init" to get started.');
    
  } catch (error) {
    console.error('❌ Failed to build rusky from source.');
    console.error('Please make sure Rust is installed: https://rustup.rs/');
    process.exit(1);
  }
}

function main() {
  console.log('🐺 Installing rusky...');
  
  // 개발 환경에서는 소스에서 빌드
  if (fs.existsSync(path.join(__dirname, 'Cargo.toml'))) {
    buildFromSource();
  } else {
    downloadBinary();
  }
}

if (require.main === module) {
  main();
} 