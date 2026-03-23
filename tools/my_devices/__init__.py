from __future__ import annotations

from ..base import BaseTool
from .switch import SwitchPowerTool
from .light import SetBrightnessTool, SetColorTool, SetColorTemperatureTool
from .curtain import ControlCurtainTool
from .tv import TvPowerTool, TvVolumeTool, TvMuteTool, TvChannelTool, TvInputTool
from .media import MediaPlaybackTool, MediaVolumeTool
from .climate import AcControlTool
from .air_quality import AirPurifierTool, DehumidifierTool
from .oven import OvenStatusTool
from .sensor import GetSensorDataTool
from .energy import GetEnergyDataTool, GetBatteryStatusTool


def get_my_device_tools() -> list[type[BaseTool]]:
    return [
        SwitchPowerTool,
        SetBrightnessTool,
        SetColorTool,
        SetColorTemperatureTool,
        ControlCurtainTool,
        TvPowerTool,
        TvVolumeTool,
        TvMuteTool,
        TvChannelTool,
        TvInputTool,
        MediaPlaybackTool,
        MediaVolumeTool,
        AcControlTool,
        AirPurifierTool,
        DehumidifierTool,
        OvenStatusTool,
        GetSensorDataTool,
        GetEnergyDataTool,
        GetBatteryStatusTool,
    ]
