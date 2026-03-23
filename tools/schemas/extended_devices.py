from __future__ import annotations

from typing import Any, Literal

from pydantic import BaseModel, Field


class RobotVacuumInput(BaseModel):
    device_id: str = Field(description="로봇청소기 디바이스 ID")
    action: Literal["start", "pause", "returnToHome"] = Field(
        description="동작 (청소 시작, 일시정지, 충전기 복귀)"
    )
    cleaning_mode: str | None = Field(
        default=None, description="청소 모드 (auto, part, repeat, manual)"
    )
    turbo: Literal["on", "off", "silence"] | None = Field(
        default=None, description="터보 모드"
    )


class DoorLockInput(BaseModel):
    device_id: str = Field(description="도어록 디바이스 ID")
    action: Literal["lock", "unlock"] = Field(description="잠금 또는 해제")


class WasherInput(BaseModel):
    device_id: str = Field(description="세탁기 디바이스 ID")
    action: Literal["run", "pause", "stop"] = Field(
        description="동작 (가동, 일시정지, 정지)"
    )
    mode: str | None = Field(
        default=None, description="세탁 모드 (regular, heavy, rinse, spinDry)"
    )


class DryerInput(BaseModel):
    device_id: str = Field(description="건조기 디바이스 ID")
    action: Literal["run", "pause", "stop"] = Field(description="동작")
    mode: str | None = Field(
        default=None, description="건조 모드 (regular, lowHeat, highHeat)"
    )


class DishwasherInput(BaseModel):
    device_id: str = Field(description="식기세척기 디바이스 ID")
    action: Literal["run", "pause", "stop"] = Field(description="동작")
    mode: str | None = Field(
        default=None, description="세척 모드 (auto, eco, intense, quick, rinse)"
    )


class RefrigeratorInput(BaseModel):
    device_id: str = Field(description="냉장고 디바이스 ID")
    feature: Literal["rapidCooling", "rapidFreezing", "defrost"] = Field(
        description="기능"
    )
    state: Literal["on", "off"] = Field(description="활성화 여부")


class ThermostatInput(BaseModel):
    device_id: str = Field(description="온도조절기 디바이스 ID")
    mode: str | None = Field(default=None, description="모드 (auto, cool, heat, off)")
    heating_setpoint: float | None = Field(
        default=None, description="난방 목표 온도 (°C)"
    )
    cooling_setpoint: float | None = Field(
        default=None, description="냉방 목표 온도 (°C)"
    )
    fan_mode: str | None = Field(
        default=None, description="팬 모드 (auto, on, circulate)"
    )


class AlarmInput(BaseModel):
    device_id: str = Field(description="경보 디바이스 ID")
    action: Literal["siren", "strobe", "both", "off"] = Field(description="경보 동작")


class SecuritySystemInput(BaseModel):
    device_id: str = Field(description="보안 시스템 디바이스 ID")
    action: Literal["armAway", "armStay", "disarm"] = Field(description="보안 모드")


class GarageDoorInput(BaseModel):
    device_id: str = Field(description="차고문 디바이스 ID")
    action: Literal["open", "close"] = Field(description="차고문 동작")


class ValveInput(BaseModel):
    device_id: str = Field(description="밸브 디바이스 ID")
    action: Literal["open", "close"] = Field(description="밸브 동작")


class SafetySensorInput(BaseModel):
    device_id: str = Field(description="안전 센서 디바이스 ID")
