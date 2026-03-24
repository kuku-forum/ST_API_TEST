from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import AirPurifierInput, DehumidifierInput


class AirPurifierTool(BaseTool):
    name = "air_purifier_control"
    description = "공기청정기를 제어합니다. 전원과 운전 모드를 설정할 수 있습니다."
    args_schema = AirPurifierInput

    def execute(self, args: AirPurifierInput) -> ToolResult:
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
                "airPurifierFanMode",
                "setAirPurifierFanMode",
                [args.mode],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set mode"),
                    data=results,
                )
            results["mode"] = response

        return ToolResult(success=True, data=results)


class DehumidifierTool(BaseTool):
    name = "dehumidifier_control"
    description = "제습기를 제어합니다. 전원, 모드, 목표 습도를 설정할 수 있습니다."
    args_schema = DehumidifierInput

    def execute(self, args: DehumidifierInput) -> ToolResult:
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
                "samsungce.dehumidifierMode",
                "setDehumidifierMode",
                [args.mode],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get("message", "Failed to set mode"),
                    data=results,
                )
            results["mode"] = response

        if args.target_humidity is not None:
            response = self.client.command(
                args.device_id,
                "samsungce.relativeHumidityLevel",
                "setHumidityLevel",
                [args.target_humidity],
            )
            if isinstance(response, dict) and "error" in response:
                return ToolResult(
                    success=False,
                    error=response["error"].get(
                        "message", "Failed to set target humidity"
                    ),
                    data=results,
                )
            results["target_humidity"] = response

        return ToolResult(success=True, data=results)
