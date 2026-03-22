# Samsung SmartThings API 테스트 도구

삼성 SmartThings REST API의 전체 엔드포인트를 카테고리별로 테스트하고, 실제 요청/응답 데이터를 확인하는 CLI 도구입니다. **Python**과 **Rust** 두 가지 구현이 동일한 기능을 제공합니다.

향후 **MCP 도구**로 활용하기 위한 기반 프로젝트입니다.

## 빠른 시작

```bash
# Python 실행
cd python && pip install -r requirements.txt && python -m src

# Rust 실행
cd rust && cargo run

# 결과 비교
python scripts/merge_logs.py
```

## 사전 준비

### 1. PAT (Personal Access Token) 발급

1. [https://account.smartthings.com/tokens](https://account.smartthings.com/tokens) 접속
2. Samsung 계정으로 로그인
3. **"Generate new token"** → 아래 권한(Scopes) 모두 선택:

| Scope | 용도 |
|-------|------|
| `r:devices:*` / `w:devices:*` / `x:devices:*` | 디바이스 조회/제어 |
| `r:locations:*` | 위치/방/모드 조회 |
| `r:scenes:*` / `x:scenes:*` | 씬 조회/실행 |
| `r:rules:*` | 규칙 조회 |
| `r:installedapps` | 설치된 앱 조회 |

4. 토큰 복사 → `.env` 파일에 저장

> ⚠️ PAT 토큰은 **24시간** 후 만료. 만료 시 재발급.

### 2. 환경변수 설정

```bash
cp .env.example .env
# .env 파일 편집: SMARTTHINGS_PAT=발급받은-토큰
```

## 사용법

### Python

```bash
cd python
python -m src                    # 전체 테스트
python -m src devices            # 카테고리별
python -m src -s                 # 부수효과 포함 (기기 제어)
python -m src --no-log           # 로그 저장 안 함
```

### Rust

```bash
cd rust
cargo run                        # 전체 테스트
cargo run -- devices             # 카테고리별
cargo run -- -s                  # 부수효과 포함
cargo run -- --no-log            # 로그 저장 안 함
```

### 로그 비교

```bash
# latest 로그 비교 (Python vs Rust)
python scripts/merge_logs.py

# 결과 파일로 저장
python scripts/merge_logs.py --save

# 특정 로그 파일 비교
python scripts/merge_logs.py --py python/logs/2026-03-22_12-00-00.json \
                             --rs rust/logs/2026-03-22_12-00-00.json
```

## API 카테고리 (15개, 29개 엔드포인트)

| 카테고리 | ID | 테스트 | 설명 |
|----------|-----|--------|------|
| 📍 위치 | `locations` | 2 | 위치 목록/상세 |
| 🚪 방 | `rooms` | 2 | 방 목록/상세 |
| 🔄 모드 | `modes` | 2 | 모드 목록/현재 |
| 📱 디바이스 | `devices` | 5 | 목록/상세/상태/건강/명령 |
| 🔧 프로필 | `deviceProfiles` | 2 | 디바이스 프로필 |
| ⚡ 기능 | `capabilities` | 3 | 기능 정의 |
| 🎬 씬 | `scenes` | 2 | 씬 목록/실행 |
| 📐 규칙 | `rules` | 2 | 자동화 규칙 |
| 📦 앱 | `apps` | 2 | SmartApp |
| 📲 설치된 앱 | `installedApps` | 2 | 설치된 앱 |
| 🔔 구독 | `subscriptions` | 1 | 이벤트 구독 |
| ⏰ 스케줄 | `schedules` | 1 | 예약 실행 |
| 🔌 스키마 | `schema` | 1 | C2C 커넥터 |
| 🌤️ 서비스 | `services` | 1 | 날씨 정보 |
| 📜 기록 | `history` | 1 | 이벤트 기록 |

## 프로젝트 구조

```
ST_API_TEST/
├── .env                         # PAT 토큰 (공유, gitignored)
├── .env.example
├── python/
│   ├── requirements.txt
│   ├── logs/                    # Python 테스트 로그
│   └── src/
│       ├── __main__.py          # python -m src 엔트리
│       ├── index.py             # CLI + 테스트 러너
│       ├── client.py            # requests HTTP 클라이언트
│       ├── models.py            # dataclass 정의
│       ├── reporter.py          # ANSI 출력
│       ├── logger.py            # JSON 로그 저장
│       └── api/                 # 15개 API 모듈
├── rust/
│   ├── Cargo.toml
│   ├── logs/                    # Rust 테스트 로그
│   └── src/
│       ├── main.rs              # CLI + 테스트 러너
│       ├── client.rs            # ureq HTTP 클라이언트
│       ├── models.rs            # struct 정의
│       ├── reporter.rs          # ANSI 출력
│       ├── logger.rs            # JSON 로그 저장
│       └── api/                 # 5개 그룹 API 모듈
└── scripts/
    ├── merge_logs.py            # Python/Rust 로그 비교 도구
    └── merged/                  # 병합 결과 저장
```

## 참고 링크

- [SmartThings 개발자 문서](https://developer.smartthings.com/docs/)
- [SmartThings API 레퍼런스](https://developer.smartthings.com/docs/api/public)
- [PAT 토큰 관리](https://account.smartthings.com/tokens)
