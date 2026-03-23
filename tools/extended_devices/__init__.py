from __future__ import annotations

from ..base import BaseTool
from .vacuum import RobotVacuumTool
from .lock import DoorLockTool
from .laundry import WasherTool, DryerTool
from .dishwasher import DishwasherTool
from .refrigerator import RefrigeratorTool
from .thermostat import ThermostatTool
from .security import AlarmTool, SecuritySystemTool
from .door_valve import GarageDoorTool, ValveTool
from .safety_sensor import SmokeDetectorTool, CoDetectorTool, WaterLeakTool


def get_extended_device_tools() -> list[type[BaseTool]]:
    return [
        RobotVacuumTool,
        DoorLockTool,
        WasherTool,
        DryerTool,
        DishwasherTool,
        RefrigeratorTool,
        ThermostatTool,
        AlarmTool,
        SecuritySystemTool,
        GarageDoorTool,
        ValveTool,
        SmokeDetectorTool,
        CoDetectorTool,
        WaterLeakTool,
    ]
