from __future__ import annotations

from typing import Any

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import (
    SetBrightnessInput,
    SetColorInput,
    SetColorTemperatureInput,
)


class SetBrightnessTool(BaseTool):
    name = "set_brightness"
    description = "조명 밝기를 0~100 사이 값으로 설정합니다. 라이트스트립, 디머 조명 등에 사용합니다."
    args_schema = SetBrightnessInput

    def execute(self, *, device_id: str, level: int, **kwargs: Any) -> ToolResult:
        data = self.client.command(device_id, "switchLevel", "setLevel", [level])
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "level": level})


class SetColorTool(BaseTool):
    name = "set_color"
    description = (
        "조명 색상을 변경합니다. hue(색조 0~100)와 saturation(채도 0~100)을 설정합니다."
    )
    args_schema = SetColorInput

    def execute(
        self, *, device_id: str, hue: int, saturation: int, **kwargs: Any
    ) -> ToolResult:
        data = self.client.command(
            device_id,
            "colorControl",
            "setColor",
            [{"hue": hue, "saturation": saturation}],
        )
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(
            success=True,
            data={"device_id": device_id, "hue": hue, "saturation": saturation},
        )


class SetColorTemperatureTool(BaseTool):
    name = "set_color_temperature"
    description = "조명 색온도를 변경합니다. 2700K(따뜻한 노란색)~6500K(차가운 흰색) 범위를 지원합니다."
    args_schema = SetColorTemperatureInput

    def execute(self, *, device_id: str, temperature: int, **kwargs: Any) -> ToolResult:
        data = self.client.command(
            device_id,
            "colorTemperature",
            "setColorTemperature",
            [temperature],
        )
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(
            success=True, data={"device_id": device_id, "temperature": temperature}
        )
