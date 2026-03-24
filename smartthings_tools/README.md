# SmartThings LLM Tool Package

## 1. 개요 (Overview)
SmartThings REST API를 LLM agent tool calling용으로 래핑한 Python 패키지입니다. Pydantic BaseModel 기반 스키마와 class 기반 도구 구조를 사용합니다. 프레임워크에 종속되지 않아 OpenAI, Anthropic, LangChain 등 어디서든 쓸 수 있습니다. 총 38개 도구(공통 5, 사용자 디바이스 19, 확장 디바이스 14)를 제공합니다.

## 2. 설치 (Installation)
의존성: pydantic>=2.0.0, requests>=2.31.0, python-dotenv>=1.0.0
```bash
# 프로젝트 루트(ST_API_TEST/)에서 실행
uv sync
```

## 3. 빠른 시작 (Quick Start)
기본 사용법은 다음과 같습니다.
```python
from smartthings_tools import SmartThingsToolkit

toolkit = SmartThingsToolkit(token="your-pat-token")
# 확장 디바이스 포함 시:
toolkit = SmartThingsToolkit(token="...", include_extended=True)

# 사용 가능한 도구 목록 확인
print(toolkit.list_tool_names())

# 도구 실행
result = toolkit.execute("list_devices")
result = toolkit.execute("switch_power", device_id="xxx", state="on")

# LLM용 스키마 가져오기
openai_tools = toolkit.to_openai_tools()
anthropic_tools = toolkit.to_anthropic_tools()
```

## 4. LLM 연동 예시 (LLM Integration Examples)

### OpenAI
```python
import openai
import os
import json
from smartthings_tools import SmartThingsToolkit

toolkit = SmartThingsToolkit(token=os.environ["SMARTTHINGS_PAT"])

response = openai.chat.completions.create(
    model="gpt-4o",
    messages=[{"role": "user", "content": "거실 조명 꺼줘"}],
    tools=toolkit.to_openai_tools(),
)

# 도구 호출 처리
for call in response.choices[0].message.tool_calls or []:
    result = toolkit.execute(call.function.name, **json.loads(call.function.arguments))
```

### Anthropic
```python
import anthropic
import os
from smartthings_tools import SmartThingsToolkit

toolkit = SmartThingsToolkit(token=os.environ["SMARTTHINGS_PAT"])

response = anthropic.messages.create(
    model="claude-sonnet-4-20250514",
    messages=[{"role": "user", "content": "잠자리 준비해줘"}],
    tools=toolkit.to_anthropic_tools(),
)

for block in response.content:
    if block.type == "tool_use":
        result = toolkit.execute(block.name, **block.input)
```

## 5. 테스트 (Testing)

```bash
# 자동 샘플 테스트 — 디바이스 조회, 센서 읽기, 날씨 등 자동 수행
uv run python smartthings_tools/examples/quick_test.py

# 수동 대화형 테스트 — 도구 선택 → 파라미터 입력 → 실행
uv run python smartthings_tools/examples/interactive.py

# 통합 테스트 — 38개 도구 등록/스키마/API 호출 검증
uv run python smartthings_tools/examples/test_my_devices.py
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
| action | `"open"` \| `"close"` \| `"pause"` | 필수 | 동작 |
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
smartthings_tools/
├── __init__.py              # SmartThingsToolkit (진입점)
├── base.py                  # BaseTool (추상 기반), ToolResult
├── client.py                # SmartThingsClient (HTTP)
├── common_tools.py          # 공통 도구 5개
├── schemas/
│   ├── common.py            # 공통 input 스키마
│   ├── my_devices.py        # 사용자 디바이스 스키마
│   └── extended_devices.py  # 확장 디바이스 스키마
├── my_devices/              # 사용자 디바이스 도구 19개
│   ├── switch.py, light.py, curtain.py
│   ├── tv.py, media.py
│   ├── climate.py, air_quality.py
│   ├── oven.py, sensor.py, energy.py
│   └── __init__.py
├── extended_devices/        # 확장 디바이스 도구 14개
│   ├── vacuum.py, lock.py, laundry.py
│   ├── dishwasher.py, refrigerator.py
│   ├── thermostat.py, security.py
│   ├── door_valve.py, safety_sensor.py
│   └── __init__.py
└── examples/
    └── test_my_devices.py   # 테스트 스크립트
```

## 8. 커스텀 도구 추가 (Adding Custom Tools)
도구를 확장하는 방법은 다음과 같습니다.
```python
from pydantic import BaseModel, Field
from typing import Literal
from smartthings_tools.base import BaseTool, ToolResult

class MyCustomInput(BaseModel):
    device_id: str = Field(description="디바이스 ID")
    param: str = Field(description="파라미터 설명")

class MyCustomTool(BaseTool):
    name = "my_custom_tool"
    description = "커스텀 도구 설명"
    args_schema = MyCustomInput

    def execute(self, *, device_id: str, param: str, **kwargs) -> ToolResult:
        data = self.client.command(device_id, "capability", "command", [param])
        if "error" in data:
            return ToolResult(success=False, error=str(data["error"]))
        return ToolResult(success=True, data=data)
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

## 10. 라이선스 (License)
ST_API_TEST 프로젝트의 일부입니다.
