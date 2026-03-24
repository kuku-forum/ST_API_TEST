from __future__ import annotations

from typing import Any

from .base import BaseTool, ToolResult
from .schemas.common import (
    ExecuteSceneInput,
    GetDeviceStatusInput,
    GetWeatherInput,
    ListDevicesInput,
    SendCommandInput,
)


class ListDevicesTool(BaseTool):
    name = "list_devices"
    description = (
        "등록된 모든 SmartThings 디바이스 목록을 조회합니다. "
        "각 디바이스의 ID, 이름, 방, 지원 capability를 반환합니다."
    )
    args_schema = ListDevicesInput

    def execute(self, input_data: ListDevicesInput | dict[str, Any]) -> ToolResult:
        if isinstance(input_data, dict):
            input_data = ListDevicesInput(**input_data)

        params: dict[str, Any] = {}
        if input_data.location_id:
            params["locationId"] = input_data.location_id
        if input_data.capability:
            params["capability"] = input_data.capability

        response = self.client.get("/devices", params=params or None)
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=str(
                    response.get("error", "디바이스 목록 조회 중 오류가 발생했습니다.")
                ),
                data=response,
            )

        items = response.get("items", []) if isinstance(response, dict) else []
        devices: list[dict[str, Any]] = []
        for item in items:
            components = item.get("components") or []
            first_component = components[0] if components else {}
            capability_ids = [
                capability.get("id")
                for capability in first_component.get("capabilities", [])
                if isinstance(capability, dict) and capability.get("id")
            ]
            devices.append(
                {
                    "device_id": item.get("deviceId"),
                    "label": item.get("label"),
                    "room_id": item.get("roomId"),
                    "capabilities": capability_ids,
                }
            )

        return ToolResult(
            success=True, data={"devices": devices, "count": len(devices)}
        )


class GetDeviceStatusTool(BaseTool):
    name = "get_device_status"
    description = "특정 디바이스의 현재 상태를 조회합니다. 전원, 온도, 밝기 등 모든 capability 상태를 반환합니다."
    args_schema = GetDeviceStatusInput

    def execute(self, input_data: GetDeviceStatusInput | dict[str, Any]) -> ToolResult:
        if isinstance(input_data, dict):
            input_data = GetDeviceStatusInput(**input_data)

        response = self.client.get(f"/devices/{input_data.device_id}/status")
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=str(
                    response.get("error", "디바이스 상태 조회 중 오류가 발생했습니다.")
                ),
                data=response,
            )

        components = (
            response.get("components", {}) if isinstance(response, dict) else {}
        )
        main = components.get("main", {}) if isinstance(components, dict) else {}

        flattened_status: dict[str, Any] = {}
        if isinstance(main, dict):
            for capability, attributes in main.items():
                if not isinstance(attributes, dict):
                    continue
                for attribute, payload in attributes.items():
                    if isinstance(payload, dict) and "value" in payload:
                        flattened_status[f"{capability}.{attribute}"] = payload.get(
                            "value"
                        )

        return ToolResult(
            success=True,
            data={"device_id": input_data.device_id, "status": flattened_status},
        )


class SendCommandTool(BaseTool):
    name = "send_command"
    description = (
        "범용 디바이스 커맨드를 전송합니다. "
        "다른 도구로 해결되지 않는 특수한 capability/command 조합에 사용합니다."
    )
    args_schema = SendCommandInput

    def execute(self, input_data: SendCommandInput | dict[str, Any]) -> ToolResult:
        if isinstance(input_data, dict):
            input_data = SendCommandInput(**input_data)

        response = self.client.command(
            device_id=input_data.device_id,
            capability=input_data.capability,
            command=input_data.command,
            arguments=input_data.arguments,
            component=input_data.component,
        )
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=str(response.get("error", "커맨드 전송 중 오류가 발생했습니다.")),
                data=response,
            )

        return ToolResult(
            success=True, data={"device_id": input_data.device_id, "result": response}
        )


class ExecuteSceneTool(BaseTool):
    name = "execute_scene"
    description = "SmartThings에 등록된 씬을 실행합니다. 여러 디바이스를 한 번에 제어할 수 있습니다."
    args_schema = ExecuteSceneInput

    def execute(self, input_data: ExecuteSceneInput | dict[str, Any]) -> ToolResult:
        if isinstance(input_data, dict):
            input_data = ExecuteSceneInput(**input_data)

        response = self.client.post(f"/scenes/{input_data.scene_id}/execute", body={})
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=str(response.get("error", "씬 실행 중 오류가 발생했습니다.")),
                data=response,
            )

        return ToolResult(
            success=True, data={"scene_id": input_data.scene_id, "result": response}
        )


class GetWeatherTool(BaseTool):
    name = "get_weather"
    description = "현재 위치의 날씨 정보(기온, 습도, 날씨 상태)를 조회합니다."
    args_schema = GetWeatherInput

    def execute(self, input_data: GetWeatherInput | dict[str, Any]) -> ToolResult:
        if isinstance(input_data, dict):
            input_data = GetWeatherInput(**input_data)

        response = self.client.get(
            f"/services/coordinate/locations/{input_data.location_id}/weather"
        )
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=str(
                    response.get("error", "날씨 정보 조회 중 오류가 발생했습니다.")
                ),
                data=response,
            )

        return ToolResult(success=True, data=response)


def get_common_tools() -> list[type[BaseTool]]:
    return [
        ListDevicesTool,
        GetDeviceStatusTool,
        SendCommandTool,
        ExecuteSceneTool,
        GetWeatherTool,
    ]
