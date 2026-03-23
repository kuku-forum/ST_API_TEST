from __future__ import annotations

from pydantic import BaseModel, Field


class ListDevicesInput(BaseModel):
    location_id: str | None = Field(
        default=None, description="조회할 위치(Location)의 ID"
    )
    capability: str | None = Field(default=None, description="필터링할 capability ID")


class GetDeviceStatusInput(BaseModel):
    device_id: str = Field(description="상태를 조회할 디바이스 ID")


class SendCommandInput(BaseModel):
    device_id: str = Field(description="커맨드를 전송할 디바이스 ID")
    capability: str = Field(description="전송할 capability ID")
    command: str = Field(description="실행할 command 이름")
    arguments: list = Field(default_factory=list, description="커맨드 인자 목록")
    component: str = Field(default="main", description="커맨드를 적용할 컴포넌트 이름")


class ExecuteSceneInput(BaseModel):
    scene_id: str = Field(description="실행할 씬(Scene) ID")


class GetWeatherInput(BaseModel):
    location_id: str = Field(description="날씨를 조회할 위치(Location) ID")
