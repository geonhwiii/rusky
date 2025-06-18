# Product Context: rusky

## 왜 rusky가 필요한가?

### 기존 문제점 (husky의 한계)
1. **성능 이슈**
   - Node.js 런타임 오버헤드 (~100ms 시작 시간)
   - 메모리 사용량 높음 (~20MB+)
   - 패키지 크기 큼 (~20MB+)

2. **의존성 문제**
   - 다수의 Node.js 의존성
   - 보안 취약점 가능성
   - 설치 시간 오래 걸림

3. **사용자 경험**
   - Git hooks 실행 시 눈에 띄는 지연
   - 대형 프로젝트에서 성능 저하
   - CI/CD 환경에서 불필요한 오버헤드

### rusky가 해결하는 문제
1. **성능 혁신**
   - 시작 시간 <1ms
   - 메모리 사용량 ~1MB
   - 바이너리 크기 ~2MB

2. **의존성 제거**
   - 런타임 의존성 제로
   - 단일 바이너리 배포
   - 빠른 설치

3. **개발자 경험 향상**
   - 즉각적인 hook 실행
   - 안정적인 성능
   - 간단한 설정

## 사용자 워크플로우

### 기본 사용 시나리오
1. **프로젝트 설정**
   ```bash
   npm install --save-dev rusky
   npx rusky init
   ```

2. **Hook 추가**
   ```bash
   npx rusky add pre-commit "npm test"
   npx rusky add pre-push "npm run lint"
   ```

3. **일상적 사용**
   - Git commit/push 시 자동 실행
   - 빠른 피드백
   - 안정적인 성능

### 고급 사용 시나리오
- 여러 hooks 관리
- CI/CD 환경에서 활용
- 팀 전체 설정 공유

## 사용자 경험 목표

### 핵심 가치
1. **속도**: 사용자가 지연을 느끼지 않는 수준
2. **신뢰성**: 항상 예상대로 작동
3. **단순함**: 복잡한 설정 불필요
4. **호환성**: 기존 husky 사용자 쉬운 마이그레이션

### 성공 지표
- Hook 실행 시간 <10ms
- 설치 시간 <30초
- 사용자 만족도 >90%
- GitHub 스타 수 증가

## 경쟁 분석

### vs husky
| 항목 | rusky | husky |
|------|-------|-------|
| 시작 시간 | <1ms | ~100ms |
| 메모리 | ~1MB | ~20MB |
| 바이너리 크기 | ~2MB | ~20MB+ |
| 의존성 | 0개 | 다수 |

### vs pre-commit
- pre-commit은 Python 기반으로 다른 사용 패턴
- rusky는 Node.js 생태계에 특화
- 더 간단한 설정과 사용법

## 마케팅 포지셔닝
"The blazingly fast Git hooks manager written in Rust - A drop-in replacement for husky with zero runtime dependencies" 