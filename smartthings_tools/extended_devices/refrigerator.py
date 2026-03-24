from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import RefrigeratorInput


class RefrigeratorTool(BaseTool):
    name = "refrigerator_control"
    description = "냉장고의 급속냉각, 급속냉동, 제상 기능을 제어합니다."
    args_schema = RefrigeratorInput

    def execute(self, params: RefrigeratorInput) -> ToolResult:
        command_by_feature = {
            "rapidCooling": "setRapidCooling",
            "rapidFreezing": "setRapidFreezing",
            "defrost": "setDefrost",
        }

        data = self.client.command(
            params.device_id,
            "refrigeration",
            command_by_feature[params.feature],
            arguments=[params.state],
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
