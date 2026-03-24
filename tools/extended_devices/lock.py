from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import DoorLockInput


class DoorLockTool(BaseTool):
    name = "door_lock_control"
    description = "스마트 도어록을 잠그거나 해제합니다."
    args_schema = DoorLockInput

    def execute(self, params: DoorLockInput) -> ToolResult:
        data = self.client.command(params.device_id, "lock", params.action)
        if "error" in data:
            error = data.get("error", {})
            message = (
                error.get("message", str(data))
                if isinstance(error, dict)
                else str(error)
            )
            return ToolResult(success=False, error=message)

        return ToolResult(success=True, data=data)
