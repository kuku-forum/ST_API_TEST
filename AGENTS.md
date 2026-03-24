# PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-22

## OVERVIEW

Samsung SmartThings REST API 테스트/탐색 CLI 도구 및 LLM 도구 패키지.
- `api_explorer/`: Python 3.9+ / requests 기반 API 테스트 CLI. 15개 API 카테고리, 29개 엔드포인트 테스트.
- `smartthings_tools/`: LLM(Large Language Model)에서 사용하기 위한 SmartThings 도구 패키지.
- `api_explorer_rs/`: Rust 구현체 (동일 기능 제공).

## STRUCTURE

```
ST_API_TEST/
├── api_explorer/       # API 테스트 CLI (Python)
│   ├── __main__.py     # python -m api_explorer 엔트리 (dotenv 로드)
│   ├── index.py        # CLI 엔트리포인트 (argparse, test runner)
│   ├── client.py       # requests 기반 HTTP 클라이언트 → api.smartthings.com/v1
│   ├── models.py       # dataclass 정의 (ApiEndpointTest, TestContext 등)
│   ├── reporter.py     # ANSI 컬러 출력 포매터
│   ├── logger.py       # JSON 로그 파일 저장 (logs/ 디렉토리)
│   └── api/            # 카테고리별 API 테스트 모듈
│       └── __init__.py # 전체 테스트 레지스트리
├── smartthings_tools/  # LLM용 도구 패키지
├── api_explorer_rs/    # API 테스트 CLI (Rust)
├── .env                # PAT 토큰 (gitignored)
└── .env.example        # 환경변수 가이드
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| API 테스트 추가 | `api_explorer/api/` — 기존 모듈 패턴 따라 작성 후 `api/__init__.py`에 등록 |
| 새 카테고리 추가 | `api_explorer/api/새카테고리.py` 생성 → `api/__init__.py`에 import 추가 |
| 출력 형식 변경 | `api_explorer/reporter.py` |
| HTTP 클라이언트 수정 | `api_explorer/client.py` |
| CLI 옵션 추가 | `api_explorer/index.py` > `argparse` 설정 |
| 환경변수 | `.env` / `.env.example` |
| 타입/모델 | `api_explorer/models.py` |
| SmartThings 도구 | `smartthings_tools/` |

## CONVENTIONS

- 각 API 모듈은 `list[ApiEndpointTest]`를 `tests` 이름으로 export
- 테스트 간 데이터 공유: `ctx.store.set('key', value)` / `ctx.store.get('key')`
- 부수효과 테스트: `has_side_effect=True` 설정 → `--side-effects` 플래그 필요
- 데이터 부재 시 건너뛰기: `needs_setup` 문자열 설정
- 한국어 출력 기본, 카테고리 라벨에 이모지 사용
- 파일명은 snake_case (예: `device_profiles.py`, `installed_apps.py`)

## ANTI-PATTERNS

- `.env`에 PAT 직접 커밋 금지 (.gitignored)
- SmartThings SDK 미사용 — requests 의도적 선택 (request/response 가시성)
- `# type: ignore` 사용 금지

## COMMANDS

```bash
uv sync                            # 의존성 설치 (최초 1회)
uv run python -m api_explorer               # 전체 테스트
uv run python -m api_explorer devices       # 카테고리별 테스트
uv run python -m api_explorer -s            # 부수효과 포함
uv run python -m api_explorer --no-log      # 로그 저장 안 함
```
