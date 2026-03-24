# Samsung SmartThings API 테스트 도구

삼성 SmartThings REST API의 전체 엔드포인트를 카테고리별로 테스트하고, 실제 요청/응답 데이터를 확인하는 CLI 도구 및 **smartthings_tools** 라이브러리입니다. **Python**과 **Rust** 두 가지 구현이 동일한 기능을 제공합니다.

향후 **MCP 도구**로 활용하기 위한 기반 프로젝트입니다.

## 빠른 시작

```bash
# Python 실행 (uv 사용)
uv sync && uv run python -m api_explorer

# Rust 실행
cd api_explorer_rs && cargo run

# 결과 비교
uv run python scripts/merge_logs.py
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
uv sync                              # 의존성 설치 (최초 1회)
uv run python -m api_explorer                 # 전체 테스트
uv run python -m api_explorer devices         # 카테고리별
uv run python -m api_explorer -s              # 부수효과 포함 (기기 제어)
uv run python -m api_explorer --no-log        # 로그 저장 안 함
```

### Rust

```bash
cd api_explorer_rs
cargo run                        # 전체 테스트
cargo run -- devices             # 카테고리별
cargo run -- -s                  # 부수효과 포함
cargo run -- --no-log            # 로그 저장 안 함
```

### 로그 비교

```bash
# latest 로그 비교 (Python vs Rust)
uv run python scripts/merge_logs.py

# 결과 파일로 저장
uv run python scripts/merge_logs.py --save

# 특정 로그 파일 비교
uv run python scripts/merge_logs.py --py api_explorer/logs/2026-03-22_12-00-00.json \
                                    --rs api_explorer_rs/logs/2026-03-22_12-00-00.json
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

---

## 아키텍처 — 각 파일의 역할

Python과 Rust는 동일한 구조를 공유합니다. 아래는 각 파일의 역할 설명입니다.

### 핵심 모듈

| 파일 (Python / Rust) | 역할 |
|---|---|
| `models.py` / `models.rs` | **데이터 모델 정의.** `ApiCallResult`(요청/응답 쌍), `ApiEndpointTest`(테스트 정의), `TestContext`(실행 컨텍스트 + 공유 저장소), `EndpointTestResult`(개별 결과), `TestSuiteResult`(전체 결과) 등 모든 타입을 정의합니다. |
| `client.py` / `client.rs` | **SmartThings HTTP 클라이언트.** `Authorization: Bearer {PAT}` 헤더를 자동 설정하고, 모든 요청의 메서드·URL·본문·응답 상태·응답 바디·소요 시간을 `ApiCallResult`로 캡처합니다. Python은 `requests`, Rust는 `ureq`(blocking) 사용. |
| `index.py` / `main.rs` | **CLI 엔트리포인트 + 테스트 러너.** 커맨드라인 인자를 파싱하고(카테고리 필터, `--side-effects`, `--no-log`), 등록된 테스트를 순서대로 실행하며, 결과를 reporter와 logger에 전달합니다. |
| `reporter.py` / `reporter.rs` | **ANSI 컬러 터미널 출력.** 카테고리별 헤더/푸터, 개별 테스트 결과(요청 URL, 응답 JSON), 엔드포인트별 상세 테이블, 카테고리별 요약 테이블을 포맷팅합니다. |
| `logger.py` / `logger.rs` | **JSON 로그 파일 저장.** 실행 결과를 `logs/{타임스탬프}.json` + `logs/latest.json`으로 저장합니다. 로그에는 `runner` 필드("python" 또는 "rust")가 포함되어 병합 스크립트에서 구분 가능합니다. |
| `__main__.py` | **Python 패키지 실행 엔트리.** `python -m api_explorer` 명령 시 `.env`를 로드하고 `index.main()`을 호출합니다. |

### API 모듈

각 모듈은 `tests` 리스트를 export하며, 테스트 간 데이터를 `ctx.store`로 공유합니다.  
예: `locations` 목록 조회 → `locationId` 저장 → `rooms` 모듈에서 사용.

**Python** (`api_explorer/api/`) — 카테고리 1:1 매핑, 15개 파일:

| 파일 | 담당 API |
|------|----------|
| `locations.py` | `GET /locations`, `GET /locations/{id}` |
| `rooms.py` | `GET /locations/{id}/rooms`, `GET .../rooms/{roomId}` |
| `modes.py` | `GET /locations/{id}/modes`, `GET .../modes/current` |
| `devices.py` | `GET /devices`, `GET /{id}`, `GET /{id}/status`, `GET /{id}/health`, `POST /{id}/commands` |
| `device_profiles.py` | `GET /deviceprofiles`, `GET /deviceprofiles/{id}` |
| `capabilities.py` | `GET /capabilities/namespaces`, `GET /capabilities/{id}/{ver}` |
| `scenes.py` | `GET /scenes`, `POST /scenes/{id}/execute` |
| `rules.py` | `GET /rules?locationId=`, `GET /rules/{id}` |
| `apps.py` | `GET /apps`, `GET /apps/{id}` |
| `installed_apps.py` | `GET /installedapps`, `GET /installedapps/{id}` |
| `subscriptions.py` | `GET /installedapps/{id}/subscriptions` |
| `schedules.py` | `GET /installedapps/{id}/schedules` |
| `schema_connectors.py` | `GET /schema/apps` |
| `services.py` | `GET /services/coordinate/locations/{id}/weather` |
| `history.py` | `GET /history/devices` |

**Rust** (`api_explorer_rs/src/api/`) — 관련 카테고리 그룹화, 5개 파일:

| 파일 | 담당 API |
|------|----------|
| `locations.rs` | 위치 + 방 + 모드 (6개 엔드포인트) |
| `devices.rs` | 디바이스 + 프로필 + 기능 (10개 엔드포인트) |
| `automations.rs` | 씬 + 규칙 (4개 엔드포인트) |
| `apps.rs` | 앱 + 설치된 앱 + 구독 + 스케줄 (6개 엔드포인트) |
| `misc.rs` | 스키마 + 서비스 + 기록 (3개 엔드포인트) |

### 스크립트

| 파일 | 역할 |
|------|------|
| `scripts/merge_logs.py` | api_explorer와 api_explorer_rs의 `logs/latest.json`을 읽어 테스트별 결과/응답 시간을 나란히 비교하고, 불일치 항목을 하이라이트합니다. `--save` 옵션으로 병합 결과를 JSON 파일로 저장 가능. |

### 데이터 흐름

```
.env (PAT 토큰)
  ↓
client (HTTP 요청/응답 캡처)
  ↓
api/* (카테고리별 테스트 실행, ctx.store로 ID 공유)
  ↓
index/main (테스트 러너 — 순차 실행, 결과 수집)
  ↓
├── reporter (터미널 ANSI 출력)
└── logger (JSON 로그 파일 저장)
       ↓
    merge_logs.py (api_explorer/api_explorer_rs 결과 비교)
```

---

## 수동 테스트 가이드

SmartThings API를 직접 호출해보고 싶다면, 아래 코드를 복사해서 사용하세요.

### Python으로 직접 호출

```python
import requests

TOKEN = "your-pat-token-here"
BASE = "https://api.smartthings.com/v1"
HEADERS = {"Authorization": f"Bearer {TOKEN}", "Accept": "application/json"}

# 1. 위치 목록 조회
resp = requests.get(f"{BASE}/locations", headers=HEADERS)
print(resp.json())
location_id = resp.json()["items"][0]["locationId"]

# 2. 디바이스 목록 조회
resp = requests.get(f"{BASE}/devices", headers=HEADERS)
devices = resp.json()["items"]
for d in devices:
    print(f"  {d['label']} ({d['deviceId']})")

# 3. 특정 디바이스 상태 조회
device_id = devices[0]["deviceId"]
resp = requests.get(f"{BASE}/devices/{device_id}/status", headers=HEADERS)
print(resp.json())

# 4. 디바이스 명령 실행 (refresh — 안전)
resp = requests.post(
    f"{BASE}/devices/{device_id}/commands",
    headers={**HEADERS, "Content-Type": "application/json"},
    json={"commands": [{"component": "main", "capability": "refresh", "command": "refresh"}]},
)
print(resp.status_code, resp.json())

# 5. 씬 목록 + 실행
resp = requests.get(f"{BASE}/scenes", headers=HEADERS)
scenes = resp.json()["items"]
if scenes:
    scene_id = scenes[0]["sceneId"]
    resp = requests.post(f"{BASE}/scenes/{scene_id}/execute", headers=HEADERS)
    print(f"씬 실행: {resp.status_code}")
```

### curl로 직접 호출

```bash
TOKEN="your-pat-token-here"

# 위치 목록
curl -s -H "Authorization: Bearer $TOKEN" \
  https://api.smartthings.com/v1/locations | python -m json.tool

# 디바이스 목록
curl -s -H "Authorization: Bearer $TOKEN" \
  https://api.smartthings.com/v1/devices | python -m json.tool

# 디바이스 상태 (device_id를 위에서 확인한 값으로 교체)
curl -s -H "Authorization: Bearer $TOKEN" \
  https://api.smartthings.com/v1/devices/{device_id}/status | python -m json.tool

# 디바이스 명령 실행
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"commands":[{"component":"main","capability":"refresh","command":"refresh"}]}' \
  https://api.smartthings.com/v1/devices/{device_id}/commands
```

### API 응답 구조 참고

```jsonc
// 목록 조회 공통 구조
{
  "items": [ ... ],          // 결과 배열
  "_links": {                // 페이지네이션
    "next": null,
    "previous": null
  }
}

// 디바이스 상태 구조
{
  "components": {
    "main": {
      "switch": {            // capability 이름
        "switch": {          // attribute 이름
          "value": "on",     // 현재 값
          "timestamp": "..."
        }
      }
    }
  }
}

// 에러 응답 구조
{
  "requestId": "...",
  "error": {
    "code": "ERR_NOT_FOUND",
    "message": "A resource could not be found.",
    "details": []
  }
}
```

---

## 프로젝트 구조

```
ST_API_TEST/
├── .env                         # PAT 토큰 (공유, gitignored)
├── .env.example                 # 환경변수 템플릿 + 발급 가이드
├── pyproject.toml               # Python 의존성 (uv sync)
├── api_explorer/                # SmartThings API 탐색/테스트 CLI (Python)
│   ├── __main__.py              # python -m api_explorer 엔트리
│   ├── index.py                 # CLI 파싱 + 테스트 러너
│   ├── client.py                # SmartThings HTTP 클라이언트
│   ├── models.py                # 데이터 모델 (dataclass)
│   ├── reporter.py              # ANSI 컬러 터미널 출력
│   ├── logger.py                # JSON 로그 저장
│   ├── api/                     # 카테고리별 API 테스트 (15개 파일)
│   └── logs/                    # 테스트 결과 JSON 로그 (gitignored)
├── api_explorer_rs/             # SmartThings API 탐색/테스트 CLI (Rust)
│   ├── Cargo.toml               # ureq, serde, clap, dotenvy
│   ├── logs/                    # 테스트 결과 JSON 로그 (gitignored)
│   └── src/
│       ├── main.rs, client.rs, models.rs, reporter.rs, logger.rs
│       └── api/                 # 그룹화된 API 테스트 (5개 파일)
├── smartthings_tools/           # LLM Agent용 SmartThings 도구 패키지 (38개 도구)
│   ├── base.py, client.py       # BaseTool, SmartThingsClient
│   ├── schemas/                 # Pydantic input 스키마
│   ├── my_devices/              # 사용자 디바이스 도구 (19개)
│   ├── extended_devices/        # 확장 디바이스 도구 (14개)
│   └── examples/                # 테스트 스크립트
└── scripts/
    └── merge_logs.py            # Python/Rust 로그 비교 도구
```

## 참고 링크

- [SmartThings 개발자 문서](https://developer.smartthings.com/docs/)
- [SmartThings API 레퍼런스](https://developer.smartthings.com/docs/api/public)
- [PAT 토큰 관리](https://account.smartthings.com/tokens)
