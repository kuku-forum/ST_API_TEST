from __future__ import annotations

from typing import Any

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import (
    TvChannelInput,
    TvInputInput,
    TvMuteInput,
    TvPowerInput,
    TvVolumeInput,
)


class TvPowerTool(BaseTool):
    name = "tv_power"
    description = "TV 전원을 켜거나 끕니다."
    args_schema = TvPowerInput

    def execute(self, *, device_id: str, state: str, **kwargs: Any) -> ToolResult:
        data = self.client.command(device_id, "switch", state)
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "state": state})


class TvVolumeTool(BaseTool):
    name = "tv_volume"
    description = "TV 볼륨을 조절합니다. 직접 설정, 올리기, 내리기를 지원합니다."
    args_schema = TvVolumeInput

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


class TvMuteTool(BaseTool):
    name = "tv_mute"
    description = "TV 음소거를 설정하거나 해제합니다."
    args_schema = TvMuteInput

    def execute(self, *, device_id: str, state: str, **kwargs: Any) -> ToolResult:
        data = self.client.command(device_id, "audioMute", state)
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "state": state})


class TvChannelTool(BaseTool):
    name = "tv_channel"
    description = (
        "TV 채널을 변경합니다. 채널 번호 직접 입력, 다음/이전 채널 이동을 지원합니다."
    )
    args_schema = TvChannelInput

    def execute(
        self,
        *,
        device_id: str,
        action: str,
        channel: str | None = None,
        **kwargs: Any,
    ) -> ToolResult:
        if action == "set":
            if channel is None:
                return ToolResult(
                    success=False, error="channel is required when action is 'set'."
                )
            data = self.client.command(
                device_id, "tvChannel", "setTvChannel", [channel]
            )
        elif action == "up":
            data = self.client.command(device_id, "tvChannel", "channelUp")
        elif action == "down":
            data = self.client.command(device_id, "tvChannel", "channelDown")
        else:
            return ToolResult(
                success=False, error=f"Unsupported channel action: {action}"
            )

        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "channel": channel},
        )


class TvInputTool(BaseTool):
    name = "tv_input"
    description = "TV 입력 소스를 변경합니다 (HDMI1, HDMI2, USB, digitalTv 등)."
    args_schema = TvInputInput

    def execute(self, *, device_id: str, source: str, **kwargs: Any) -> ToolResult:
        data = self.client.command(
            device_id, "mediaInputSource", "setInputSource", [source]
        )
        if "error" in data:
            return ToolResult(
                success=False, error=data.get("error", {}).get("message", str(data))
            )
        return ToolResult(success=True, data={"device_id": device_id, "source": source})
