from __future__ import annotations

from tools.base import BaseTool, ToolResult
from tools.schemas.extended_devices import AlarmInput, SecuritySystemInput


def _extract_error_message(data: dict) -> str:
    error_detail = data.get("error", {})
    if isinstance(error_detail, dict):
        return error_detail.get("message", str(error_detail))
    return str(error_detail)


class AlarmTool(BaseTool):
    name = "alarm_control"
    description = "사이렌/경보 디바이스를 제어합니다. 사이렌, 스트로브, 동시작동, 끄기를 지원합니다."
    args_schema = AlarmInput

    def execute(self, device_id: str, action: str) -> ToolResult:
        data = self.client.command(device_id, "alarm", action)
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "result": data},
        )


class SecuritySystemTool(BaseTool):
    name = "security_system_control"
    description = (
        "보안 시스템의 경계 모드를 설정합니다. 외출 경계, 재실 경계, 해제를 지원합니다."
    )
    args_schema = SecuritySystemInput

    def execute(self, device_id: str, action: str) -> ToolResult:
        data = self.client.command(device_id, "securitySystem", action)
        if "error" in data:
            return ToolResult(success=False, error=_extract_error_message(data))
        return ToolResult(
            success=True,
            data={"device_id": device_id, "action": action, "result": data},
        )
