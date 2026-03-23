from __future__ import annotations

from typing import Any

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import MediaPlaybackInput, MediaVolumeInput


class MediaPlaybackTool(BaseTool):
    name = "media_playback"
    description = (
        "미디어 재생을 제어합니다. 재생, 일시정지, 정지, 빨리감기, 되감기를 지원합니다."
    )
    args_schema = MediaPlaybackInput

    def execute(self, *, device_id: str, action: str, **kwargs: Any) -> ToolResult:
        data = self.client.command(device_id, "mediaPlayback", action)
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "action": action})


class MediaVolumeTool(BaseTool):
    name = "media_volume"
    description = "미디어 디바이스(스피커 등)의 볼륨을 조절합니다."
    args_schema = MediaVolumeInput

    def execute(
        self,
        *,
        device_id: str,
        action: str,
        volume: int | None = None,
        **kwargs: Any,
    ) -> ToolResult:
        if action == "set":
            if volume is None:
                return ToolResult(
                    success=False, error="volume is required when action is 'set'."
                )
            data = self.client.command(device_id, "audioVolume", "setVolume", [volume])
        elif action == "up":
            data = self.client.command(device_id, "audioVolume", "volumeUp")
        elif action == "down":
            data = self.client.command(device_id, "audioVolume", "volumeDown")
        else:
            return ToolResult(
                success=False, error=f"Unsupported volume action: {action}"
            )

        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "volume": volume},
        )
