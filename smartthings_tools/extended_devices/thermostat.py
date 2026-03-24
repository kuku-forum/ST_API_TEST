from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.extended_devices import ThermostatInput


def _extract_error_message(data: dict) -> str:
    error_detail = data.get("error", {})
    if isinstance(error_detail, dict):
        return error_detail.get("message", str(error_detail))
    return str(error_detail)


class ThermostatTool(BaseTool):
    name = "thermostat_control"
    description = "온도조절기/보일러를 제어합니다. 운전 모드, 난방 목표 온도, 냉방 목표 온도, 팬 모드를 설정할 수 있습니다."
    args_schema = ThermostatInput

    def execute(
        self,
        device_id: str,
        mode: str | None = None,
        heating_setpoint: float | int | None = None,
        cooling_setpoint: float | int | None = None,
        fan_mode: str | None = None,
    ) -> ToolResult:
        commands: list[tuple[str, str, str, list[object], object]] = []

        if mode is not None:
            commands.append(
                ("mode", "thermostatMode", "setThermostatMode", [mode], mode)
            )
        if heating_setpoint is not None:
            commands.append(
                (
                    "heating_setpoint",
                    "thermostatHeatingSetpoint",
                    "setHeatingSetpoint",
                    [heating_setpoint],
                    heating_setpoint,
                )
            )
        if cooling_setpoint is not None:
            commands.append(
                (
                    "cooling_setpoint",
                    "thermostatCoolingSetpoint",
                    "setCoolingSetpoint",
                    [cooling_setpoint],
                    cooling_setpoint,
                )
            )
        if fan_mode is not None:
            commands.append(
                (
                    "fan_mode",
                    "thermostatFanMode",
                    "setThermostatFanMode",
                    [fan_mode],
                    fan_mode,
                )
            )

        if not commands:
            return ToolResult(success=False, error="설정할 항목이 없습니다.")

        applied: dict[str, object] = {}
        responses: dict[str, dict] = {}

        for key, capability, command, arguments, value in commands:
            data = self.client.command(device_id, capability, command, arguments)
            if "error" in data:
                return ToolResult(success=False, error=_extract_error_message(data))
            applied[key] = value
            responses[key] = data

        return ToolResult(
            success=True,
            data={
                "device_id": device_id,
                "set": applied,
                "results": responses,
            },
        )
