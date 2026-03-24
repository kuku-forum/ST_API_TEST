#!/usr/bin/env python3
"""SmartThings Tool Package 통합 테스트 스크립트.

실제 SmartThings API를 호출하여 도구 패키지가 정상 작동하는지 검증한다.
.env 파일에 SMARTTHINGS_PAT 토큰이 필요하다.

사용법::

    uv run python smartthings_tools/examples/test_my_devices.py
"""

from __future__ import annotations

import json
import os
import sys

from dotenv import find_dotenv, load_dotenv

load_dotenv(find_dotenv())

from smartthings_tools import SmartThingsToolkit, ToolResult

PASS = "\033[92mPASS\033[0m"
FAIL = "\033[91mFAIL\033[0m"
SKIP = "\033[93mSKIP\033[0m"


def print_result(test_name: str, result: ToolResult) -> None:
    status = PASS if result.success else FAIL
    print(f"  [{status}] {test_name}")
    if result.success and result.data:
        preview = json.dumps(result.data, ensure_ascii=False, default=str)
        if len(preview) > 200:
            preview = preview[:200] + "..."
        print(f"         {preview}")
    elif not result.success:
        print(f"         error: {result.error}")


def main() -> None:
    token = os.getenv("SMARTTHINGS_PAT")
    if not token:
        print(f"[{FAIL}] SMARTTHINGS_PAT 환경변수가 설정되지 않았습니다.")
        print("       .env 파일에 SMARTTHINGS_PAT=your-token 을 추가하세요.")
        sys.exit(1)

    toolkit = SmartThingsToolkit(token=token, include_extended=True)
    total_tools = len(toolkit.get_tools())
    print(f"\nSmartThings Tool Package 테스트")
    print(f"등록된 도구: {total_tools}개")
    print(f"{'=' * 50}\n")

    passed = 0
    failed = 0
    skipped = 0

    print("[1] 도구 등록 검증")
    names = toolkit.list_tool_names()
    print(f"  [{PASS}] {len(names)}개 도구 등록 확인")
    passed += 1

    print("\n[2] 스키마 변환 검증")
    openai_schemas = toolkit.to_openai_tools()
    assert all("type" in s and s["type"] == "function" for s in openai_schemas)
    print(f"  [{PASS}] OpenAI 형식 변환 ({len(openai_schemas)}개)")
    passed += 1

    anthropic_schemas = toolkit.to_anthropic_tools()
    assert all("input_schema" in s for s in anthropic_schemas)
    print(f"  [{PASS}] Anthropic 형식 변환 ({len(anthropic_schemas)}개)")
    passed += 1

    print("\n[3] API 호출 테스트 (실제 SmartThings API)")

    result = toolkit.execute("list_devices")
    print_result("list_devices", result)
    if result.success:
        passed += 1
    else:
        failed += 1

    device_id = None
    sensor_id = None
    plug_id = None

    if result.success and result.data:
        devices = (
            result.data.get("devices", [])
            if isinstance(result.data, dict)
            else result.data
        )
        print(f"         발견된 디바이스: {len(devices)}개")

        for d in devices:
            caps = d.get("capabilities", [])
            if not device_id and "switch" in caps:
                device_id = d["device_id"]
            if not sensor_id and any(
                c in caps
                for c in [
                    "temperatureMeasurement",
                    "presenceSensor",
                    "motionSensor",
                ]
            ):
                sensor_id = d["device_id"]
            if not plug_id and "powerMeter" in caps:
                plug_id = d["device_id"]

    if device_id:
        result = toolkit.execute("get_device_status", device_id=device_id)
        print_result(f"get_device_status ({device_id[:8]}...)", result)
        if result.success:
            passed += 1
        else:
            failed += 1
    else:
        print(f"  [{SKIP}] get_device_status (switch 디바이스 없음)")
        skipped += 1

    if sensor_id:
        result = toolkit.execute("get_sensor_data", device_id=sensor_id)
        print_result(f"get_sensor_data ({sensor_id[:8]}...)", result)
        if result.success:
            passed += 1
        else:
            failed += 1
    else:
        print(f"  [{SKIP}] get_sensor_data (센서 디바이스 없음)")
        skipped += 1

    if plug_id:
        result = toolkit.execute("get_energy_data", device_id=plug_id)
        print_result(f"get_energy_data ({plug_id[:8]}...)", result)
        if result.success:
            passed += 1
        else:
            failed += 1
    else:
        print(f"  [{SKIP}] get_energy_data (플러그 디바이스 없음)")
        skipped += 1

    loc_result = toolkit.execute("list_devices")
    if loc_result.success and loc_result.data:
        from smartthings_tools.client import SmartThingsClient

        client = SmartThingsClient(token)
        loc_data = client.get("/locations")
        items = loc_data.get("items", [])
        if items:
            location_id = items[0]["locationId"]
            result = toolkit.execute("get_weather", location_id=location_id)
            print_result(f"get_weather ({location_id[:8]}...)", result)
            if result.success:
                passed += 1
            else:
                failed += 1
        else:
            print(f"  [{SKIP}] get_weather (위치 정보 없음)")
            skipped += 1
    else:
        print(f"  [{SKIP}] get_weather (API 접근 실패)")
        skipped += 1

    print("\n[4] 입력 검증 테스트")

    bad_result = toolkit.execute("switch_power", device_id="", state="invalid")
    if not bad_result.success:
        print(f"  [{PASS}] 잘못된 입력 거부됨: {bad_result.error[:80]}")
        passed += 1
    else:
        print(f"  [{FAIL}] 잘못된 입력이 통과됨")
        failed += 1

    unknown_result = toolkit.execute("nonexistent_tool")
    if not unknown_result.success:
        print(f"  [{PASS}] 미등록 도구 거부됨: {unknown_result.error}")
        passed += 1
    else:
        print(f"  [{FAIL}] 미등록 도구가 통과됨")
        failed += 1

    print(f"\n{'=' * 50}")
    total = passed + failed + skipped
    print(f"결과: {passed}/{total} 통과, {failed} 실패, {skipped} 건너뜀")

    if failed > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
