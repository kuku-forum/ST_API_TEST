# SmartThings LLM Tool Package

## 1. 개요 (Overview)
SmartThings REST API를 LLM agent tool calling용으로 래핑한 Python 패키지입니다. Pydantic BaseModel 기반 스키마와 class 기반 도구 구조를 사용합니다. 프레임워크에 종속되지 않아 OpenAI, Anthropic, LangChain 등 어디서든 쓸 수 있습니다. 총 38개 도구(공통 5, 사용자 디바이스 19, 확장 디바이스 14)를 제공합니다.

## 2. 설치 (Installation)
의존성: pydantic>=2.0.0, requests>=2.31.0, python-dotenv>=1.0.0
```bash
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

## 5. 도구 목록 (Tool Reference)

### 공통 도구 (5개)
| 도구 이름 | 설명 | 주요 파라미터 |
|-----------|------|-------------|
| list_devices | 디바이스 목록 조회 | location_id?, capability? |
| get_device_status | 디바이스 상태 조회 | device_id |
| send_command | 범용 커맨드 전송 | device_id, capability, command, arguments |
| execute_scene | 씬 실행 | scene_id |
| get_weather | 날씨 조회 | location_id |

### 사용자 디바이스 도구 (19개)
| 도구 이름 | 설명 | 주요 파라미터 |
|-----------|------|-------------|
| switch_power | 전원 on/off | device_id, state |
| set_brightness | 밝기 설정 | device_id, level (0-100) |
| set_color | 색상 변경 | device_id, hue, saturation |
| set_color_temperature | 색온도 변경 | device_id, temperature (2700-6500K) |
| control_curtain | 커튼 제어 | device_id, action, level? |
| tv_power | TV 전원 | device_id, state |
| tv_volume | TV 볼륨 | device_id, action, volume? |
| tv_mute | TV 음소거 | device_id, state |
| tv_channel | TV 채널 | device_id, action, channel? |
| tv_input | TV 입력소스 | device_id, source |
| media_playback | 미디어 재생 제어 | device_id, action |
| media_volume | 미디어 볼륨 | device_id, action, volume? |
| ac_control | 에어컨 제어 | device_id, power?, mode?, temperature?, wind_level? |
| air_purifier_control | 공기청정기 제어 | device_id, power?, mode? |
| dehumidifier_control | 제습기 제어 | device_id, power?, mode?, target_humidity? |
| oven_status | 오븐 상태 조회 | device_id |
| get_sensor_data | 센서 데이터 조회 | device_id |
| get_energy_data | 전력 사용량 조회 | device_id |
| get_battery_status | 배터리 잔량 조회 | device_id |

### 확장 디바이스 도구 (14개)
| 도구 이름 | 설명 | 주요 파라미터 |
|-----------|------|-------------|
| robot_vacuum_control | 로봇청소기 제어 | device_id, action, cleaning_mode?, turbo? |
| door_lock_control | 도어록 제어 | device_id, action |
| washer_control | 세탁기 제어 | device_id, action, mode? |
| dryer_control | 건조기 제어 | device_id, action, mode? |
| dishwasher_control | 식기세척기 제어 | device_id, action, mode? |
| refrigerator_control | 냉장고 제어 | device_id, feature, state |
| thermostat_control | 온도조절기 제어 | device_id, mode?, heating_setpoint?, cooling_setpoint?, fan_mode? |
| alarm_control | 경보 제어 | device_id, action |
| security_system_control | 보안시스템 제어 | device_id, action |
| garage_door_control | 차고문 제어 | device_id, action |
| valve_control | 밸브 제어 | device_id, action |
| smoke_detector_status | 화재감지 상태 조회 | device_id |
| co_detector_status | CO 감지 상태 조회 | device_id |
| water_leak_status | 누수 감지 상태 조회 | device_id |

## 6. 아키텍처 (Architecture)
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

## 7. 커스텀 도구 추가 (Adding Custom Tools)
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

## 8. SmartThings API 참조 (API Reference)
- Base URL: https://api.smartthings.com/v1
- Auth: Bearer {PAT Token}
- PAT 발급: https://account.smartthings.com/tokens
- 주요 capability와 command 매핑 테이블:
  - switch: on/off
  - switchLevel: setLevel
  - windowShade: open/close/pause
  - thermostatMode: setThermostatMode
  - airConditionerMode: setAirConditionerMode

## 9. 라이선스 (License)
ST_API_TEST 프로젝트의 일부입니다.
