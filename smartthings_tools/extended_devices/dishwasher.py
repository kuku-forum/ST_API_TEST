from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import DishwasherInput


class DishwasherTool(BaseTool):
    name = "dishwasher_control"
    description = "식기세척기를 제어합니다. 가동, 일시정지, 정지 및 세척 모드를 설정할 수 있습니다."
    args_schema = DishwasherInput

    def execute(self, params: DishwasherInput) -> ToolResult:
        data = self.client.command(
            params.device_id,
            "dishwasherOperatingState",
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
                "dishwasherMode",
                "setDishwasherMode",
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
