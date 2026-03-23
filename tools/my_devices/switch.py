from __future__ import annotations

from typing import Any

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import SwitchPowerInput


class SwitchPowerTool(BaseTool):
    name = "switch_power"
    description = "디바이스 전원을 켜거나 끕니다. 조명, 스마트플러그, 공기청정기, 제습기, TV, 환풍기 등 switch capability가 있는 모든 디바이스에 사용합니다."
    args_schema = SwitchPowerInput

    def execute(self, *, device_id: str, state: str, **kwargs: Any) -> ToolResult:
        data = self.client.command(device_id, "switch", state)
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "state": state})
