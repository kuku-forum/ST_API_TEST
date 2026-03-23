from __future__ import annotations

from tools.base import BaseTool, ToolResult
from tools.schemas.my_devices import GetBatteryInput, GetEnergyDataInput


class GetEnergyDataTool(BaseTool):
    name = "get_energy_data"
    description = "스마트 플러그의 전력 사용량을 조회합니다. 현재 전력(W)과 누적 에너지(kWh)를 반환합니다."
    args_schema = GetEnergyDataInput

    def execute(self, args: GetEnergyDataInput) -> ToolResult:
        response = self.client.get(f"/devices/{args.device_id}/status")
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=response["error"].get("message", "Failed to get energy data"),
            )

        main = (
            response.get("components", {}).get("main", {})
            if isinstance(response, dict)
            else {}
        )

        data = {
            "power_w": main.get("powerMeter", {}).get("power", {}).get("value"),
            "energy_kwh": main.get("energyMeter", {}).get("energy", {}).get("value"),
        }
        return ToolResult(success=True, data=data)


class GetBatteryStatusTool(BaseTool):
    name = "get_battery_status"
    description = "배터리 구동 디바이스의 배터리 잔량을 확인합니다."
    args_schema = GetBatteryInput

    def execute(self, args: GetBatteryInput) -> ToolResult:
        response = self.client.get(f"/devices/{args.device_id}/status")
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=response["error"].get("message", "Failed to get battery status"),
            )

        main = (
            response.get("components", {}).get("main", {})
            if isinstance(response, dict)
            else {}
        )
        data = {
            "battery_percent": main.get("battery", {}).get("battery", {}).get("value")
        }
        return ToolResult(success=True, data=data)
