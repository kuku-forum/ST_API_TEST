from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import GarageDoorInput, ValveInput


def _extract_error_message(data: dict) -> str:
    error_detail = data.get("error", {})
    if isinstance(error_detail, dict):
        return error_detail.get("message", str(error_detail))
    return str(error_detail)


class GarageDoorTool(BaseTool):
    name = "garage_door_control"
    description = "차고문을 열거나 닫습니다."
    args_schema = GarageDoorInput

    def execute(self, device_id: str, action: str) -> ToolResult:
        data = self.client.command(device_id, "doorControl", action)
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "result": data},
        )


class ValveTool(BaseTool):
    name = "valve_control"
    description = "스마트 밸브(수도 차단 밸브 등)를 열거나 닫습니다."
    args_schema = ValveInput

    def execute(self, device_id: str, action: str) -> ToolResult:
        data = self.client.command(device_id, "valve", action)
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "result": data},
        )
