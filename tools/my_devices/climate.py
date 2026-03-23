from __future__ import annotations

from tools.base import BaseTool, ToolResult
from tools.schemas.my_devices import AcControlInput


class AcControlTool(BaseTool):
    name = "ac_control"
    description = "에어컨을 제어합니다. 전원, 모드, 온도, 풍량을 설정할 수 있습니다."
    args_schema = AcControlInput

    def execute(self, args: AcControlInput) -> ToolResult:
        results: dict[str, object] = {}

        if args.power is not None:
            response = self.client.command(args.device_id, "switch", args.power)
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set power"),
                    data=results,
                )
            results["power"] = response

        if args.mode is not None:
            response = self.client.command(
                args.device_id,
                "airConditionerMode",
                "setAirConditionerMode",
                [args.mode],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set mode"),
                    data=results,
                )
            results["mode"] = response

        if args.temperature is not None:
            response = self.client.command(
                args.device_id,
                "thermostatCoolingSetpoint",
                "setCoolingSetpoint",
                [args.temperature],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set temperature"),
                    data=results,
                )
            results["temperature"] = response

        if args.wind_level is not None:
            response = self.client.command(
                args.device_id,
                "airConditionerFanMode",
                "setFanMode",
                [str(args.wind_level)],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set wind level"),
                    data=results,
                )
            results["wind_level"] = response

        return ToolResult(success=True, data=results)
