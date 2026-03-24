#!/usr/bin/env python3
"""SmartThings Tool 수동 테스트 CLI.

도구를 직접 선택하고 파라미터를 입력하여 테스트할 수 있는 대화형 인터페이스.

실행::

    uv run python smartthings_tools/examples/interactive.py
"""

from __future__ import annotations

import json
import os
import sys

from dotenv import find_dotenv, load_dotenv

load_dotenv(find_dotenv())

from smartthings_tools import SmartThingsToolkit
from smartthings_tools.base import BaseTool


def pp(data: object) -> str:
    return json.dumps(data, indent=2, ensure_ascii=False, default=str)


def show_tool_info(tool: BaseTool) -> None:
    print(f"\n  도구: {tool.name}")
    print(f"  설명: {tool.description}")
    schema = tool.args_schema.model_json_schema()
    props = schema.get("properties", {})
    required = set(schema.get("required", []))

    if not props:
        print("  파라미터: 없음")
        return

    print("  파라미터:")
    for name, info in props.items():
        req = "필수" if name in required else "선택"
        ptype = info.get("type", "")
        enum = info.get("enum")
        desc = info.get("description", "")

        type_str = ptype
        if enum:
            type_str = " | ".join(f'"{v}"' for v in enum)
        elif "anyOf" in info:
            parts = []
            for sub in info["anyOf"]:
                if sub.get("type"):
                    parts.append(sub["type"])
                elif sub.get("enum"):
                    parts.append(" | ".join(f'"{v}"' for v in sub["enum"]))
            type_str = " | ".join(parts) if parts else "any"

        minimum = info.get("minimum", info.get("exclusiveMinimum"))
        maximum = info.get("maximum", info.get("exclusiveMaximum"))
        range_str = ""
        if minimum is not None or maximum is not None:
            range_str = f" ({minimum or ''}~{maximum or ''})"

        print(f"    - {name} [{req}] ({type_str}{range_str}): {desc}")


def prompt_params(tool: BaseTool) -> dict:
    schema = tool.args_schema.model_json_schema()
    props = schema.get("properties", {})
    required = set(schema.get("required", []))
    params: dict = {}

    for name, info in props.items():
        is_required = name in required
        ptype = info.get("type", "")
        enum = info.get("enum")
        default = info.get("default")

        hint = ""
        if enum:
            hint = f" ({' / '.join(str(v) for v in enum)})"
        elif ptype == "integer":
            mn = info.get("minimum", "")
            mx = info.get("maximum", "")
            if mn or mx:
                hint = f" ({mn}~{mx})"

        tag = "필수" if is_required else "선택, Enter=건너뜀"
        raw = input(f"    {name}{hint} [{tag}]: ").strip()

        if not raw:
            if is_required:
                print(f"    ! {name}은(는) 필수입니다.")
                return prompt_params(tool)
            continue

        if ptype == "integer":
            params[name] = int(raw)
        elif ptype == "number":
            params[name] = float(raw)
        elif raw.lower() in ("null", "none"):
            continue
        else:
            params[name] = raw

    return params


def main() -> None:
    token = os.getenv("SMARTTHINGS_PAT")
    if not token:
        print("SMARTTHINGS_PAT 환경변수가 없습니다. .env 파일을 확인하세요.")
        sys.exit(1)

    toolkit = SmartThingsToolkit(token=token, include_extended=True)
    tools = toolkit.get_tools()
    tool_map = {t.name: t for t in tools}

    print(f"\nSmartThings Tool 수동 테스트")
    print(f"등록된 도구: {len(tools)}개")
    print(f"종료: q 또는 Ctrl+C\n")

    categories = {
        "공통": [
            t
            for t in tools
            if t.name
            in (
                "list_devices",
                "get_device_status",
                "send_command",
                "execute_scene",
                "get_weather",
            )
        ],
        "스위치/조명": [
            t
            for t in tools
            if t.name
            in ("switch_power", "set_brightness", "set_color", "set_color_temperature")
        ],
        "커튼": [t for t in tools if t.name in ("control_curtain",)],
        "TV": [t for t in tools if t.name.startswith("tv_")],
        "미디어": [t for t in tools if t.name.startswith("media_")],
        "에어컨/공조": [
            t
            for t in tools
            if t.name in ("ac_control", "air_purifier_control", "dehumidifier_control")
        ],
        "센서/에너지": [
            t
            for t in tools
            if t.name
            in (
                "get_sensor_data",
                "get_energy_data",
                "get_battery_status",
                "oven_status",
            )
        ],
        "확장 디바이스": [
            t
            for t in tools
            if t.name
            in (
                "robot_vacuum_control",
                "door_lock_control",
                "washer_control",
                "dryer_control",
                "dishwasher_control",
                "refrigerator_control",
                "thermostat_control",
                "alarm_control",
                "security_system_control",
                "garage_door_control",
                "valve_control",
                "smoke_detector_status",
                "co_detector_status",
                "water_leak_status",
            )
        ],
    }

    while True:
        print("-" * 50)
        print("카테고리:")
        cat_names = list(categories.keys())
        for i, name in enumerate(cat_names):
            count = len(categories[name])
            print(f"  [{i}] {name} ({count}개)")
        print(f"  [a] 전체 도구 목록")

        try:
            choice = input("\n카테고리 번호 (또는 도구 이름 직접 입력): ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\n종료.")
            break

        if choice.lower() == "q":
            break

        selected_tool = None

        if choice in tool_map:
            selected_tool = tool_map[choice]
        elif choice == "a":
            print("\n전체 도구:")
            for t in tools:
                print(f"  {t.name}: {t.description}")
            continue
        elif choice.isdigit() and int(choice) < len(cat_names):
            cat = categories[cat_names[int(choice)]]
            print(f"\n{cat_names[int(choice)]}:")
            for j, t in enumerate(cat):
                print(f"  [{j}] {t.name}: {t.description}")

            try:
                tool_choice = input("도구 번호 (또는 이름): ").strip()
            except (EOFError, KeyboardInterrupt):
                print("\n종료.")
                break

            if tool_choice.isdigit() and int(tool_choice) < len(cat):
                selected_tool = cat[int(tool_choice)]
            elif tool_choice in tool_map:
                selected_tool = tool_map[tool_choice]
            else:
                print("잘못된 선택")
                continue
        else:
            print("잘못된 입력")
            continue

        if selected_tool is None:
            continue

        show_tool_info(selected_tool)
        print("\n  파라미터 입력:")
        params = prompt_params(selected_tool)

        print(f"\n  실행: {selected_tool.name}({params})")
        result = selected_tool.validate_and_execute(params)

        if result.success:
            print(f"  결과: 성공")
            print(f"  데이터:\n{pp(result.data)}")
        else:
            print(f"  결과: 실패")
            print(f"  에러: {result.error}")
        print()


if __name__ == "__main__":
    main()
