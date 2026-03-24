from __future__ import annotations

from typing import Literal

from pydantic import BaseModel, Field


class SwitchPowerInput(BaseModel):
    device_id: str = Field(description="전원을 제어할 디바이스 ID")
    state: Literal["on", "off"] = Field(description="전원 상태(on/off)")


class SetBrightnessInput(BaseModel):
    device_id: str = Field(description="밝기를 설정할 디바이스 ID")
    level: int = Field(ge=0, le=100, description="밝기 레벨(0~100)")


class SetColorInput(BaseModel):
    device_id: str = Field(description="색상을 설정할 디바이스 ID")
    hue: int = Field(ge=0, le=100, description="색조(Hue) 값(0~100)")
    saturation: int = Field(ge=0, le=100, description="채도(Saturation) 값(0~100)")


class SetColorTemperatureInput(BaseModel):
    device_id: str = Field(description="색온도를 설정할 디바이스 ID")
    temperature: int = Field(ge=2700, le=6500, description="색온도(K) 값(2700~6500)")


class ControlCurtainInput(BaseModel):
    device_id: str = Field(description="커튼을 제어할 디바이스 ID")
    action: Literal["open", "close", "pause"] = Field(
        description="커튼 동작(open/close/pause)"
    )
    level: int | None = Field(
        default=None, ge=0, le=100, description="커튼 열림 정도(0~100), 선택값"
    )


class TvPowerInput(BaseModel):
    device_id: str = Field(description="TV 전원을 제어할 디바이스 ID")
    state: Literal["on", "off"] = Field(description="TV 전원 상태(on/off)")


class TvVolumeInput(BaseModel):
    device_id: str = Field(description="TV 볼륨을 제어할 디바이스 ID")
    action: Literal["set", "up", "down"] = Field(description="볼륨 동작(set/up/down)")
    volume: int | None = Field(
        default=None,
        ge=0,
        le=100,
        description="설정할 볼륨 값(0~100), set 동작에서 사용",
    )


class TvMuteInput(BaseModel):
    device_id: str = Field(description="TV 음소거를 제어할 디바이스 ID")
    state: Literal["mute", "unmute"] = Field(description="음소거 상태(mute/unmute)")


class TvChannelInput(BaseModel):
    device_id: str = Field(description="TV 채널을 제어할 디바이스 ID")
    action: Literal["set", "up", "down"] = Field(description="채널 동작(set/up/down)")
    channel: str | None = Field(
        default=None, description="설정할 채널 번호, set 동작에서 사용"
    )


class TvInputInput(BaseModel):
    device_id: str = Field(description="TV 입력 소스를 변경할 디바이스 ID")
    source: str = Field(description="입력 소스 이름(예: HDMI1)")


class MediaPlaybackInput(BaseModel):
    device_id: str = Field(description="미디어 재생을 제어할 디바이스 ID")
    action: Literal["play", "pause", "stop", "fastForward", "rewind"] = Field(
        description="재생 동작(play/pause/stop/fastForward/rewind)"
    )


class MediaVolumeInput(BaseModel):
    device_id: str = Field(description="미디어 볼륨을 제어할 디바이스 ID")
    action: Literal["set", "up", "down"] = Field(description="볼륨 동작(set/up/down)")
    volume: int | None = Field(
        default=None,
        ge=0,
        le=100,
        description="설정할 볼륨 값(0~100), set 동작에서 사용",
    )


class AcControlInput(BaseModel):
    device_id: str = Field(description="에어컨을 제어할 디바이스 ID")
    power: Literal["on", "off"] | None = Field(
        default=None, description="전원 상태(on/off), 선택값"
    )
    mode: str | None = Field(default=None, description="운전 모드, 선택값")
    temperature: float | None = Field(default=None, description="설정 온도, 선택값")
    wind_level: int | None = Field(default=None, description="바람 세기, 선택값")


class AirPurifierInput(BaseModel):
    device_id: str = Field(description="공기청정기를 제어할 디바이스 ID")
    power: Literal["on", "off"] | None = Field(
        default=None, description="전원 상태(on/off), 선택값"
    )
    mode: str | None = Field(default=None, description="동작 모드, 선택값")


class DehumidifierInput(BaseModel):
    device_id: str = Field(description="제습기를 제어할 디바이스 ID")
    power: Literal["on", "off"] | None = Field(
        default=None, description="전원 상태(on/off), 선택값"
    )
    mode: str | None = Field(default=None, description="동작 모드, 선택값")
    target_humidity: int | None = Field(
        default=None, ge=30, le=70, description="목표 습도(30~70), 선택값"
    )


class GetSensorDataInput(BaseModel):
    device_id: str = Field(description="센서 데이터를 조회할 디바이스 ID")


class GetEnergyDataInput(BaseModel):
    device_id: str = Field(description="에너지 데이터를 조회할 디바이스 ID")


class GetBatteryInput(BaseModel):
    device_id: str = Field(description="배터리 정보를 조회할 디바이스 ID")
