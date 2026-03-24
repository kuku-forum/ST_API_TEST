#!/usr/bin/env python3
"""SmartThings Tool Package 샘플 테스트.

PAT 토큰만 있으면 바로 실행 가능한 자동 데모 스크립트.
디바이스 목록 조회 → 상태 확인 → 센서 읽기 등을 자동으로 수행한다.

실행::

    uv run python smartthings_tools/examples/quick_test.py
"""

from __future__ import annotations

import json
import os
import sys

from dotenv import find_dotenv, load_dotenv

load_dotenv(find_dotenv())

from smartthings_tools import SmartThingsToolkit


def pp(data: object) -> str:
    return json.dumps(data, indent=2, ensure_ascii=False, default=str)


def main() -> None:
    token = os.getenv("SMARTTHINGS_PAT")
    if not token:
        print("SMARTTHINGS_PAT 환경변수가 없습니다. .env 파일을 확인하세요.")
        sys.exit(1)

    toolkit = SmartThingsToolkit(token=token)
    print(f"등록된 도구: {len(toolkit.list_tool_names())}개\n")

    print("=" * 60)
    print(" 1. 디바이스 목록 조회")
    print("=" * 60)
    result = toolkit.execute("list_devices")
    if not result.success:
        print(f"실패: {result.error}")
        sys.exit(1)

    devices = result.data.get("devices", []) if isinstance(result.data, dict) else []
    print(f"발견된 디바이스: {len(devices)}개\n")

    for i, d in enumerate(devices):
        caps = ", ".join(d.get("capabilities", [])[:5])
        extra = "..." if len(d.get("capabilities", [])) > 5 else ""
        print(f"  [{i}] {d.get('label', '?')}")
        print(f"      ID: {d['device_id']}")
        print(f"      capabilities: {caps}{extra}")
        print()

    if not devices:
        print("디바이스가 없습니다. PAT 토큰이 만료되었을 수 있습니다.")
        return

    first = devices[0]
    first_id = first["device_id"]
    first_label = first.get("label", first_id[:8])

    print("=" * 60)
    print(f" 2. 디바이스 상태 조회: {first_label}")
    print("=" * 60)
    result = toolkit.execute("get_device_status", device_id=first_id)
    if result.success:
        print(pp(result.data))
    else:
        print(f"실패: {result.error}")

    sensor = next(
        (
            d
            for d in devices
            if any(
                c in d.get("capabilities", [])
                for c in ["temperatureMeasurement", "relativeHumidityMeasurement"]
            )
        ),
        None,
    )
    if sensor:
        print()
        print("=" * 60)
        print(f" 3. 센서 데이터 조회: {sensor.get('label', '?')}")
        print("=" * 60)
        result = toolkit.execute("get_sensor_data", device_id=sensor["device_id"])
        if result.success:
            print(pp(result.data))
        else:
            print(f"실패: {result.error}")

    plug = next((d for d in devices if "powerMeter" in d.get("capabilities", [])), None)
    if plug:
        print()
        print("=" * 60)
        print(f" 4. 전력 사용량 조회: {plug.get('label', '?')}")
        print("=" * 60)
        result = toolkit.execute("get_energy_data", device_id=plug["device_id"])
        if result.success:
            print(pp(result.data))
        else:
            print(f"실패: {result.error}")

    print()
    print("=" * 60)
    print(" 5. 위치 + 날씨 조회")
    print("=" * 60)
    from smartthings_tools.client import SmartThingsClient

    client = SmartThingsClient(token)
    loc_data = client.get("/locations")
    items = loc_data.get("items", [])
    if items:
        loc_id = items[0]["locationId"]
        loc_name = items[0].get("name", loc_id[:8])
        print(f"위치: {loc_name}")
        result = toolkit.execute("get_weather", location_id=loc_id)
        if result.success:
            print(pp(result.data))
        else:
            print(f"실패: {result.error}")
    else:
        print("위치 정보 없음")

    print("\n샘플 테스트 완료.")


if __name__ == "__main__":
    main()
