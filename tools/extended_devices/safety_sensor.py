from __future__ import annotations

from tools.base import BaseTool, ToolResult
from tools.schemas.extended_devices import SafetySensorInput


def _extract_error_message(data: dict) -> str:
    error_detail = data.get("error", {})
    if isinstance(error_detail, dict):
        return error_detail.get("message", str(error_detail))
    return str(error_detail)


class SmokeDetectorTool(BaseTool):
    name = "smoke_detector_status"
    description = "화재 감지기의 현재 상태를 조회합니다. (읽기 전용)"
    args_schema = SafetySensorInput

    def execute(self, device_id: str) -> ToolResult:
        data = self.client.get(f"/devices/{device_id}/status")
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))

        main = data.get("components", {}).get("main", {})
        smoke = main.get("smokeDetector", {}).get("smoke", {}).get("value")

        return ToolResult(success=True, data={"smoke": smoke, "device_id": device_id})


class CoDetectorTool(BaseTool):
    name = "co_detector_status"
    description = "일산화탄소 감지기의 현재 상태를 조회합니다. (읽기 전용)"
    args_schema = SafetySensorInput

    def execute(self, device_id: str) -> ToolResult:
        data = self.client.get(f"/devices/{device_id}/status")
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))

        main = data.get("components", {}).get("main", {})
        carbon_monoxide = (
            main.get("carbonMonoxideDetector", {})
            .get("carbonMonoxide", {})
            .get("value")
        )

        return ToolResult(
            success=True,
            data={"carbon_monoxide": carbon_monoxide, "device_id": device_id},
        )


class WaterLeakTool(BaseTool):
    name = "water_leak_status"
    description = "누수 감지 센서의 현재 상태를 조회합니다. (읽기 전용)"
    args_schema = SafetySensorInput

    def execute(self, device_id: str) -> ToolResult:
        data = self.client.get(f"/devices/{device_id}/status")
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))

        main = data.get("components", {}).get("main", {})
        water = main.get("waterSensor", {}).get("water", {}).get("value")

        return ToolResult(success=True, data={"water": water, "device_id": device_id})
