from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.common import GetDeviceStatusInput


class OvenStatusTool(BaseTool):
    name = "oven_status"
    description = "오븐/쿠커의 현재 상태를 조회합니다. 운전 모드, 동작 상태, 온도 등을 확인할 수 있습니다. (읽기 전용)"
    args_schema = GetDeviceStatusInput

    def execute(self, args: GetDeviceStatusInput) -> ToolResult:
        response = self.client.get(f"/devices/{args.device_id}/status")
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=response["error"].get("message", "Failed to get oven status"),
            )

        main = (
            response.get("components", {}).get("main", {})
            if isinstance(response, dict)
            else {}
        )

        oven_mode = main.get("ovenMode", {}).get("ovenMode", {}).get("value")
        oven_operating_state = (
            main.get("ovenOperatingState", {}).get("machineState", {}).get("value")
        )
        temperature_value = (
            main.get("temperatureMeasurement", {}).get("temperature", {}).get("value")
        )
        temperature_unit = (
            main.get("temperatureMeasurement", {}).get("temperature", {}).get("unit")
        )

        data: dict[str, object] = {
            "oven_mode": oven_mode,
            "oven_operating_state": oven_operating_state,
            "temperature": {"value": temperature_value, "unit": temperature_unit},
        }

        return ToolResult(success=True, data=data)
