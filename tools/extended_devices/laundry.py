from __future__ import annotations

from tools.base import BaseTool, ToolResult
from tools.schemas.extended_devices import DryerInput, WasherInput


class WasherTool(BaseTool):
    name = "washer_control"
    description = (
        "세탁기를 제어합니다. 가동, 일시정지, 정지 및 세탁 모드를 설정할 수 있습니다."
    )
    args_schema = WasherInput

    def execute(self, params: WasherInput) -> ToolResult:
        data = self.client.command(
            params.device_id,
            "washerOperatingState",
            "setMachineState",
            arguments=[params.action],
        )
        if "error" in data:
            error = data.get("error", {})
            message = (
                error.get("message", str(data))
                if isinstance(error, dict)
                else str(error)
            )
            return ToolResult(success=False, error=message)

        if params.mode:
            data = self.client.command(
                params.device_id,
                "washerMode",
                "setWasherMode",
                arguments=[params.mode],
            )
            if "error" in data:
                error = data.get("error", {})
                message = (
                    error.get("message", str(data))
                    if isinstance(error, dict)
                    else str(error)
                )
                return ToolResult(success=False, error=message)

        return ToolResult(success=True, data=data)


class DryerTool(BaseTool):
    name = "dryer_control"
    description = (
        "건조기를 제어합니다. 가동, 일시정지, 정지 및 건조 모드를 설정할 수 있습니다."
    )
    args_schema = DryerInput

    def execute(self, params: DryerInput) -> ToolResult:
        data = self.client.command(
            params.device_id,
            "dryerOperatingState",
            "setMachineState",
            arguments=[params.action],
        )
        if "error" in data:
            error = data.get("error", {})
            message = (
                error.get("message", str(data))
                if isinstance(error, dict)
                else str(error)
            )
            return ToolResult(success=False, error=message)

        if params.mode:
            data = self.client.command(
                params.device_id,
                "dryerMode",
                "setDryerMode",
                arguments=[params.mode],
            )
            if "error" in data:
                error = data.get("error", {})
                message = (
                    error.get("message", str(data))
                    if isinstance(error, dict)
                    else str(error)
                )
                return ToolResult(success=False, error=message)

        return ToolResult(success=True, data=data)
