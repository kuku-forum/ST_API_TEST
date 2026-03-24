from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import RobotVacuumInput


class RobotVacuumTool(BaseTool):
    name = "robot_vacuum_control"
    description = "로봇청소기를 제어합니다. 청소 시작/일시정지/복귀, 청소 모드, 터보 모드를 설정할 수 있습니다."
    args_schema = RobotVacuumInput

    def execute(self, params: RobotVacuumInput) -> ToolResult:
        movement_by_action = {
            "start": "cleaning",
            "pause": "pause",
            "returnToHome": "homing",
        }

        movement = movement_by_action[params.action]
        data = self.client.command(
            params.device_id,
            "robotCleanerMovement",
            "setRobotCleanerMovement",
            arguments=[movement],
        )
        if "error" in data:
            error = data.get("error", {})
            message = (
                error.get("message", str(data))
                if isinstance(error, dict)
                else str(error)
            )
            return ToolResult(success=False, error=message)

        if params.cleaning_mode:
            data = self.client.command(
                params.device_id,
                "robotCleanerCleaningMode",
                "setRobotCleanerCleaningMode",
                arguments=[params.cleaning_mode],
            )
            if "error" in data:
                error = data.get("error", {})
                message = (
                    error.get("message", str(data))
                    if isinstance(error, dict)
                    else str(error)
                )
                return ToolResult(success=False, error=message)

        if params.turbo:
            data = self.client.command(
                params.device_id,
                "robotCleanerTurboMode",
                "setRobotCleanerTurboMode",
                arguments=[params.turbo],
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
