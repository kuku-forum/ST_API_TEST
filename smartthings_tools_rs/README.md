# SmartThings LLM Tool Package (Rust)

## 1. 개요 (Overview)
SmartThings REST API를 LLM agent tool calling용으로 래핑한 Rust 라이브러리입니다. Python [`smartthings_tools`](../smartthings_tools/) 패키지와 동일한 기능을 제공합니다. `serde` 기반 JSON 직렬화, `schemars` 기반 JSON Schema 생성, `ureq` blocking HTTP 클라이언트를 사용합니다. 프레임워크에 종속되지 않아 OpenAI, Anthropic 등 어디서든 쓸 수 있습니다. 총 38개 도구(공통 5, 사용자 디바이스 19, 확장 디바이스 14)를 제공합니다.

## 2. 설치 (Installation)
의존성: ureq 2.x, serde 1.x, serde_json 1.x, schemars 0.8.x, dotenvy 0.15.x
```bash
# Cargo.toml에 의존성 추가
[dependencies]
smartthings_tools_rs = { path = "../smartthings_tools_rs" }

# 또는 프로젝트 디렉토리에서 직접 빌드
cd smartthings_tools_rs
cargo build
```

## 3. 빠른 시작 (Quick Start)
기본 사용법은 다음과 같습니다.
```rust
use smartthings_tools_rs::SmartThingsToolkit;
use serde_json::json;

let toolkit = SmartThingsToolkit::new("your-pat-token", false);
// 확장 디바이스 포함 시:
let toolkit = SmartThingsToolkit::new("your-pat-token", true);

// 사용 가능한 도구 목록 확인
println!("{:?}", toolkit.list_tool_names());

// 도구 실행
let result = toolkit.execute("list_devices", json!({}));
let result = toolkit.execute("switch_power", json!({
    "device_id": "xxx",
    "state": "on"
}));

// LLM용 스키마 가져오기
let openai_tools = toolkit.to_openai_tools();
let anthropic_tools = toolkit.to_anthropic_tools();
```

## 4. LLM 연동 예시 (LLM Integration Examples)

### OpenAI
```rust
use smartthings_tools_rs::SmartThingsToolkit;
use serde_json::{json, Value};

let toolkit = SmartThingsToolkit::new(&std::env::var("SMARTTHINGS_PAT").unwrap(), false);
let openai_key = std::env::var("OPENAI_API_KEY").unwrap();

let agent = ureq::AgentBuilder::new().build();
let body = json!({
    "model": "gpt-4o",
    "messages": [{"role": "user", "content": "거실 조명 꺼줘"}],
    "tools": toolkit.to_openai_tools(),
});

let resp: Value = agent
    .post("https://api.openai.com/v1/chat/completions")
    .set("Authorization", &format!("Bearer {openai_key}"))
    .send_json(body)
    .unwrap()
    .into_json()
    .unwrap();

// 도구 호출 처리
if let Some(calls) = resp["choices"][0]["message"]["tool_calls"].as_array() {
    for call in calls {
        let name = call["function"]["name"].as_str().unwrap();
        let args: Value = serde_json::from_str(
            call["function"]["arguments"].as_str().unwrap()
        ).unwrap();
        let result = toolkit.execute(name, args);
        println!("{name}: {:?}", result);
    }
}
```

### Anthropic
```rust
use smartthings_tools_rs::SmartThingsToolkit;
use serde_json::{json, Value};

let toolkit = SmartThingsToolkit::new(&std::env::var("SMARTTHINGS_PAT").unwrap(), false);
let anthropic_key = std::env::var("ANTHROPIC_API_KEY").unwrap();

let agent = ureq::AgentBuilder::new().build();
let body = json!({
    "model": "claude-sonnet-4-20250514",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "잠자리 준비해줘"}],
    "tools": toolkit.to_anthropic_tools(),
});

let resp: Value = agent
    .post("https://api.anthropic.com/v1/messages")
    .set("x-api-key", &anthropic_key)
    .set("anthropic-version", "2023-06-01")
    .send_json(body)
    .unwrap()
    .into_json()
    .unwrap();

// 도구 호출 처리
if let Some(content) = resp["content"].as_array() {
    for block in content {
        if block["type"].as_str() == Some("tool_use") {
            let name = block["name"].as_str().unwrap();
            let input = block["input"].clone();
            let result = toolkit.execute(name, input);
            println!("{name}: {:?}", result);
        }
    }
}
```

## 5. 테스트 (Testing)

```bash
# 자동 샘플 테스트 — 디바이스 조회, 센서 읽기, 날씨 등 자동 수행
cargo run --example quick_test

# 통합 테스트 — 38개 도구 등록/스키마/API 호출 검증
cargo test -- --nocapture
```

## 6. 도구 목록 (Tool Reference)

> 파라미터 표기법: **필수**는 그대로, **선택**은 `(선택)` 표시.
> 타입이 `"on" | "off"` 처럼 표기된 것은 해당 문자열만 허용하는 Literal 타입.

### 공통 도구 (5개)

**list_devices** — 등록된 모든 디바이스 목록 조회

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| location_id | str | 선택 | 특정 위치의 디바이스만 필터링 |
| capability | str | 선택 | 특정 capability를 가진 디바이스만 필터링 |

**get_device_status** — 특정 디바이스의 현재 상태 조회

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |

**send_command** — 범용 커맨드 전송 (다른 도구로 해결 안 될 때 사용)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| capability | str | 필수 | capability 이름 (예: `"switch"`, `"switchLevel"`) |
| command | str | 필수 | 명령 이름 (예: `"on"`, `"setLevel"`) |
| arguments | list | 선택 | 명령 인자 (예: `[75]`). 기본값 `[]` |
| component | str | 선택 | 컴포넌트 이름. 기본값 `"main"` |

**execute_scene** — 등록된 씬 실행

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| scene_id | str | 필수 | 씬 UUID |

**get_weather** — 현재 위치의 날씨 조회

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| location_id | str | 필수 | SmartThings 위치 UUID |

---

### 사용자 디바이스 도구 (19개)

**switch_power** — 디바이스 전원 on/off

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| state | `"on"` \| `"off"` | 필수 | 전원 상태 |

**set_brightness** — 조명 밝기 설정

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| level | int (0~100) | 필수 | 밝기 퍼센트 |

**set_color** — 조명 색상 변경

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| hue | int (0~100) | 필수 | 색조 |
| saturation | int (0~100) | 필수 | 채도 |

**set_color_temperature** — 조명 색온도 변경

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| temperature | int (2700~6500) | 필수 | 색온도 (K). 2700=따뜻한 노란색, 6500=차가운 흰색 |

**control_curtain** — 커튼/블라인드 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"open"` \| `"close"` \| `"pause"` | 선택 | 동작 |
| level | int (0~100) | 선택 | 커튼 위치 (0=완전 닫힘, 100=완전 열림) |

**tv_power** — TV 전원

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | TV 디바이스 UUID |
| state | `"on"` \| `"off"` | 필수 | 전원 상태 |

**tv_volume** — TV 볼륨 조절

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | TV 디바이스 UUID |
| action | `"set"` \| `"up"` \| `"down"` | 필수 | 볼륨 동작 |
| volume | int (0~100) | 선택 | action이 `"set"`일 때 목표 볼륨 |

**tv_mute** — TV 음소거

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | TV 디바이스 UUID |
| state | `"mute"` \| `"unmute"` | 필수 | 음소거 상태 |

**tv_channel** — TV 채널 변경

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | TV 디바이스 UUID |
| action | `"set"` \| `"up"` \| `"down"` | 필수 | 채널 동작 |
| channel | str | 선택 | action이 `"set"`일 때 채널 번호 |

**tv_input** — TV 입력소스 변경

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | TV 디바이스 UUID |
| source | str | 필수 | 입력소스 (예: `"HDMI1"`, `"HDMI2"`, `"USB"`, `"digitalTv"`) |

**media_playback** — 미디어 재생 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"play"` \| `"pause"` \| `"stop"` \| `"fastForward"` \| `"rewind"` | 필수 | 재생 동작 |

**media_volume** — 미디어 디바이스 볼륨 조절

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"set"` \| `"up"` \| `"down"` | 필수 | 볼륨 동작 |
| volume | int (0~100) | 선택 | action이 `"set"`일 때 목표 볼륨 |

**ac_control** — 에어컨 제어 (전원, 모드, 온도, 풍량)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| power | `"on"` \| `"off"` | 선택 | 전원 |
| mode | str | 선택 | 운전 모드 (예: `"cool"`, `"heat"`, `"auto"`, `"dry"`) |
| temperature | float | 선택 | 목표 온도 (°C) |
| wind_level | int | 선택 | 풍량 단계 |

**air_purifier_control** — 공기청정기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| power | `"on"` \| `"off"` | 선택 | 전원 |
| mode | str | 선택 | 운전 모드 |

**dehumidifier_control** — 제습기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| power | `"on"` \| `"off"` | 선택 | 전원 |
| mode | str | 선택 | 운전 모드 |
| target_humidity | int (30~70) | 선택 | 목표 습도 (%) |

**oven_status** — 오븐/쿠커 상태 조회 (읽기 전용)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |

**get_sensor_data** — 센서 측정값 조회 (온도, 습도, 조도, 모션, 재실 등)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 센서 디바이스 UUID |

**get_energy_data** — 스마트플러그 전력 사용량 조회

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 플러그 디바이스 UUID |

**get_battery_status** — 배터리 잔량 확인

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 배터리 구동 디바이스 UUID |

---

### 확장 디바이스 도구 (14개)

**robot_vacuum_control** — 로봇청소기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"start"` \| `"pause"` \| `"returnToHome"` | 필수 | 동작 |
| cleaning_mode | str | 선택 | 청소 모드 (예: `"auto"`, `"part"`, `"repeat"`) |
| turbo | `"on"` \| `"off"` \| `"silence"` | 선택 | 터보 모드 |

**door_lock_control** — 스마트 도어록

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"lock"` \| `"unlock"` | 필수 | 잠금/해제 |

**washer_control** — 세탁기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"run"` \| `"pause"` \| `"stop"` | 필수 | 동작 |
| mode | str | 선택 | 세탁 모드 (예: `"regular"`, `"heavy"`, `"rinse"`) |

**dryer_control** — 건조기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"run"` \| `"pause"` \| `"stop"` | 필수 | 동작 |
| mode | str | 선택 | 건조 모드 (예: `"regular"`, `"lowHeat"`, `"highHeat"`) |

**dishwasher_control** — 식기세척기 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"run"` \| `"pause"` \| `"stop"` | 필수 | 동작 |
| mode | str | 선택 | 세척 모드 (예: `"auto"`, `"eco"`, `"intense"`, `"quick"`) |

**refrigerator_control** — 냉장고 기능 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| feature | `"rapidCooling"` \| `"rapidFreezing"` \| `"defrost"` | 필수 | 기능 |
| state | `"on"` \| `"off"` | 필수 | 활성화 여부 |

**thermostat_control** — 온도조절기/보일러 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| mode | str | 선택 | 운전 모드 (예: `"auto"`, `"cool"`, `"heat"`, `"off"`) |
| heating_setpoint | float | 선택 | 난방 목표 온도 (°C) |
| cooling_setpoint | float | 선택 | 냉방 목표 온도 (°C) |
| fan_mode | str | 선택 | 팬 모드 (예: `"auto"`, `"on"`, `"circulate"`) |

**alarm_control** — 사이렌/경보 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"siren"` \| `"strobe"` \| `"both"` \| `"off"` | 필수 | 경보 동작 |

**security_system_control** — 보안 시스템 경계 모드

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"armAway"` \| `"armStay"` \| `"disarm"` | 필수 | 보안 모드 |

**garage_door_control** — 차고문 제어

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"open"` \| `"close"` | 필수 | 동작 |

**valve_control** — 스마트 밸브 (수도 차단 등)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |
| action | `"open"` \| `"close"` | 필수 | 동작 |

**smoke_detector_status** — 화재 감지기 상태 조회 (읽기 전용)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |

**co_detector_status** — 일산화탄소 감지기 상태 조회 (읽기 전용)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |

**water_leak_status** — 누수 감지 센서 상태 조회 (읽기 전용)

| 파라미터 | 타입 | 필수 | 설명 |
|----------|------|------|------|
| device_id | str | 필수 | 디바이스 UUID |

## 7. 아키텍처 (Architecture)
```
smartthings_tools_rs/
├── Cargo.toml
├── src/
│   ├── lib.rs              # SmartThingsToolkit (진입점)
│   ├── tool.rs             # Tool trait, ToolResult
│   ├── client.rs           # SmartThingsClient (ureq HTTP)
│   ├── common_tools.rs     # 공통 도구 5개
│   ├── schemas/
│   │   ├── mod.rs           # 스키마 모듈 선언
│   │   ├── common.rs       # 공통 input 스키마
│   │   ├── my_devices.rs   # 사용자 디바이스 스키마
│   │   └── extended_devices.rs  # 확장 디바이스 스키마
│   ├── my_devices/         # 사용자 디바이스 도구 19개
│   │   ├── mod.rs
│   │   ├── switch.rs, light.rs, curtain.rs
│   │   ├── tv.rs, media.rs
│   │   ├── climate.rs, air_quality.rs
│   │   ├── oven.rs, sensor.rs, energy.rs
│   │   └── battery.rs
│   └── extended_devices/   # 확장 디바이스 도구 14개
│       ├── mod.rs
│       ├── vacuum.rs, lock.rs, laundry.rs
│       ├── dishwasher.rs, refrigerator.rs
│       ├── thermostat.rs, security.rs
│       ├── door_valve.rs
│       └── safety_sensor.rs
├── tests/
│   └── integration_test.rs # 통합 테스트
└── examples/
    └── quick_test.rs       # 샘플 실행
```

## 8. 커스텀 도구 추가 (Adding Custom Tools)
도구를 확장하는 방법은 다음과 같습니다.
```rust
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use smartthings_tools_rs::client::SmartThingsClient;
use smartthings_tools_rs::tool::{Tool, ToolResult};

// 1. Input 스키마 정의
#[derive(Debug, Deserialize, JsonSchema)]
struct MyCustomInput {
    device_id: String,
    param: String,
}

// 2. Tool 구조체 정의
struct MyCustomTool<'a> {
    client: &'a SmartThingsClient,
}

// 3. Tool trait 구현
impl Tool for MyCustomTool<'_> {
    fn name(&self) -> &'static str {
        "my_custom_tool"
    }

    fn description(&self) -> &'static str {
        "커스텀 도구 설명"
    }

    fn parameters_schema(&self) -> schemars::schema::RootSchema {
        schemars::schema_for!(MyCustomInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: MyCustomInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self.client.command(
            &input.device_id,
            "capability",
            "command",
            Some(vec![Value::String(input.param)]),
            None,
        );

        if response.contains_key("error") {
            return ToolResult::fail(format!("{:?}", response.get("error")));
        }
        ToolResult::ok(serde_json::to_value(&response).unwrap_or_default())
    }
}
```

## 9. SmartThings API 참조 (API Reference)
- Base URL: https://api.smartthings.com/v1
- Auth: Bearer {PAT Token}
- PAT 발급: https://account.smartthings.com/tokens
- 주요 capability와 command 매핑 테이블:
  - switch: on/off
  - switchLevel: setLevel
  - windowShade: open/close/pause
  - thermostatMode: setThermostatMode
  - airConditionerMode: setAirConditionerMode

## 10. Python 대비 차이점

| 항목 | Python | Rust |
|------|--------|------|
| HTTP 클라이언트 | `requests` | `ureq` (blocking) |
| JSON 스키마 | Pydantic `BaseModel` | `serde` + `schemars` |
| 추상 클래스 | ABC (`BaseTool`) | Trait (`Tool`) |
| 도구 실행 | `toolkit.execute("name", **kwargs)` | `toolkit.execute("name", json!({}))` |
| 입력 검증 | Pydantic validation | `serde_json::from_value` |
| 에러 처리 | `ToolResult(success=False, error=...)` | `ToolResult::fail(...)` |

## 11. 라이선스 (License)
ST_API_TEST 프로젝트의 일부입니다.
