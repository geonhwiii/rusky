# Tech Context: rusky

## 기술 스택

### 핵심 언어 및 런타임
- **Rust 1.70+**: 시스템 프로그래밍 언어
- **Edition 2021**: 최신 Rust 에디션 사용

### 주요 의존성

#### CLI 및 사용자 인터페이스
```toml
clap = { version = "4.0", features = ["derive"] }
colored = "2.0"
```
- **clap**: CLI 인터페이스 생성 및 파싱
- **colored**: 터미널 컬러 출력

#### 비동기 런타임
```toml
tokio = { version = "1.0", features = ["full"] }
```
- **tokio**: 비동기 런타임 및 I/O 처리

#### 직렬화 및 설정 관리
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
- **serde**: 구조체 직렬화/역직렬화
- **serde_json**: JSON 형식 처리

#### 에러 처리 및 유틸리티
```toml
anyhow = "1.0"
dirs = "5.0"
```
- **anyhow**: 에러 체인 및 처리
- **dirs**: 시스템 디렉토리 경로

#### 개발 의존성
```toml
[dev-dependencies]
tempfile = "3.0"
```
- **tempfile**: 테스트용 임시 파일 생성

### npm 생태계 통합

#### package.json 설정
```json
{
  "engines": {
    "node": ">=14.0.0"
  },
  "os": ["darwin", "linux", "win32"],
  "cpu": ["x64", "arm64"]
}
```

#### 설치 스크립트 (install.js)
- Node.js 기반 설치 로직
- 플랫폼별 바이너리 다운로드
- 소스 빌드 fallback

## 개발 환경

### 필수 도구
- **Rust**: rustup을 통한 설치
- **Cargo**: Rust 패키지 매니저
- **Node.js 14+**: npm 패키지 테스트용
- **Git**: 버전 관리 및 테스트

### 빌드 설정

#### 개발 빌드
```bash
cargo build
```
- 디버그 정보 포함
- 빠른 컴파일
- 개발 및 테스트용

#### 릴리스 빌드
```bash
cargo build --release
```
- 최적화 적용
- 작은 바이너리 크기
- 배포용

### 크로스 플랫폼 지원

#### 지원 플랫폼
- **macOS**: darwin-x64, darwin-arm64
- **Linux**: linux-x64, linux-arm64  
- **Windows**: win32-x64, win32-arm64

#### 플랫폼별 고려사항
- **Unix 계열**: 파일 권한 (chmod 755)
- **Windows**: .exe 확장자
- **ARM64**: Apple Silicon 및 ARM 서버 지원

## 성능 특성

### 바이너리 크기
- **개발 빌드**: ~8MB (디버그 정보 포함)
- **릴리스 빌드**: ~1.7MB (최적화됨)
- **목표**: 5MB 이하 유지

### 메모리 사용량
- **시작 시**: ~1MB
- **실행 중**: ~1-2MB
- **목표**: 5MB 이하 유지

### 실행 성능
- **시작 시간**: <1ms
- **Hook 실행**: <10ms
- **파일 I/O**: 비동기 처리로 최적화

## 배포 전략

### npm 패키지 배포
1. **바이너리 포함**: 플랫폼별 사전 빌드된 바이너리
2. **소스 배포**: Rust 소스코드 포함
3. **Fallback**: 로컬 빌드 지원

### GitHub Releases
- 플랫폼별 바이너리 배포
- 자동화된 CI/CD 파이프라인
- 버전 태깅 및 체인지로그

## 개발 워크플로우

### 로컬 개발
```bash
# 개발 서버 실행
cargo run -- --help

# 테스트 실행
cargo test

# 린트 검사
cargo clippy

# 포맷팅
cargo fmt
```

### 테스트 전략
- **단위 테스트**: 각 모듈별 테스트
- **통합 테스트**: CLI 명령어 테스트
- **성능 테스트**: 벤치마크 측정

## 보안 고려사항

### 의존성 보안
- **cargo audit**: 취약점 스캔
- **최소 의존성**: 필요한 크레이트만 사용
- **정기 업데이트**: 보안 패치 적용

### 코드 보안
- **Rust 안전성**: 메모리 안전성 보장
- **입력 검증**: 사용자 입력 검증
- **권한 관리**: 최소 권한 원칙

## 향후 기술적 고려사항

### 성능 최적화
- **LTO 적용**: 링크 타임 최적화
- **Profile-guided optimization**: PGO 적용
- **SIMD 최적화**: 벡터 연산 활용

### 기능 확장
- **WebAssembly**: 브라우저 환경 지원
- **플러그인 시스템**: 동적 라이브러리 로딩
- **원격 설정**: 클라우드 기반 설정 동기화

### 모니터링 및 관찰성
- **텔레메트리**: 사용 패턴 수집
- **로깅**: 구조화된 로그 출력
- **메트릭**: 성능 지표 수집 