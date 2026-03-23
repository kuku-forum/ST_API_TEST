from __future__ import annotations

from typing import Any

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import ControlCurtainInput


class ControlCurtainTool(BaseTool):
    name = "control_curtain"
    description = (
        "커튼/블라인드를 열거나 닫습니다. 특정 위치(0~100%)로 설정할 수도 있습니다."
    )
    args_schema = ControlCurtainInput

    def execute(
        self,
        *,
        device_id: str,
        action: str | None = None,
        level: int | None = None,
        **kwargs: Any,
    ) -> ToolResult:
        shade_result: dict[str, Any] = {}
        level_result: dict[str, Any] = {}

        if action is not None:
            if action not in {"open", "close", "pause"}:
                return ToolResult(
                    success=False, error=f"Unsupported curtain action: {action}"
                )
            shade_result = self.client.command(device_id, "windowShade", action) or {}
            if "error" in shade_result:
                return ToolResult(
                    success=False,
                    error=shade_result.get("error", {}).get(
                        "message", str(shade_result)
                    ),
                )

        if level is not None:
            level_result = (
                self.client.command(device_id, "switchLevel", "setLevel", [level]) or {}
            )
            if "error" in level_result:
                return ToolResult(
                    success=False,
                    error=level_result.get("error", {}).get(
                        "message", str(level_result)
                    ),
                )

        if action is None and level is None:
            return ToolResult(
                success=False, error="Either action or level must be provided."
            )

        return ToolResult(
            success=True,
            data={
                "device_id": device_id,
                "action": action,
                "level": level,
                "shade_result": shade_result,
                "level_result": level_result,
            },
        )
