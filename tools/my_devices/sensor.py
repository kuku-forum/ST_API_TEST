from __future__ import annotations

from ..base import BaseTool, ToolResult
from ..schemas.my_devices import GetSensorDataInput


class GetSensorDataTool(BaseTool):
    name = "get_sensor_data"
    description = "센서 디바이스의 현재 측정값을 조회합니다. 재실/모션 감지, 온도, 습도, 조도, 접촉, 소리 감지 등 모든 센서 데이터를 반환합니다."
    args_schema = GetSensorDataInput

    def execute(self, args: GetSensorDataInput) -> ToolResult:
        response = self.client.get(f"/devices/{args.device_id}/status")
        if isinstance(response, dict) and "error" in response:
            return ToolResult(
                success=False,
                error=response["error"].get("message", "Failed to get sensor data"),
            )

        main = (
            response.get("components", {}).get("main", {})
            if isinstance(response, dict)
            else {}
        )

        raw_data = {
            "presence": main.get("presenceSensor", {}).get("presence", {}).get("value"),
            "motion": main.get("motionSensor", {}).get("motion", {}).get("value"),
            "temperature": (
                {
                    "value": main.get("temperatureMeasurement", {})
                    .get("temperature", {})
                    .get("value"),
                    "unit": main.get("temperatureMeasurement", {})
                    .get("temperature", {})
                    .get("unit"),
                }
                if main.get("temperatureMeasurement", {})
                .get("temperature", {})
                .get("value")
                is not None
                else None
            ),
            "humidity": main.get("relativeHumidityMeasurement", {})
            .get("humidity", {})
            .get("value"),
            "illuminance": main.get("illuminanceMeasurement", {})
            .get("illuminance", {})
            .get("value"),
            "contact": main.get("contactSensor", {}).get("contact", {}).get("value"),
            "sound_detection": (
                main.get("soundDetection", {}).get("soundDetected", {}).get("value")
                or main.get("soundDetection", {})
                .get("soundDetectionState", {})
                .get("value")
            ),
            "battery": main.get("battery", {}).get("battery", {}).get("value"),
        }

        data = {key: value for key, value in raw_data.items() if value is not None}
        return ToolResult(success=True, data=data)
