use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::GetSensorDataInput;
use crate::tool::{Tool, ToolResult};

pub struct GetSensorDataTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GetSensorDataTool<'_> {
    fn name(&self) -> &'static str {
        "get_sensor_data"
    }
    fn description(&self) -> &'static str {
        "센서 디바이스의 현재 측정값을 조회합니다. 재실/모션 감지, 온도, 습도, 조도, 접촉, 소리 감지 등 모든 센서 데이터를 반환합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GetSensorDataInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GetSensorDataInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&response) {
            return ToolResult::fail(extract_error_msg(&response));
        }

        let main = response.get("components").and_then(|c| c.get("main"));

        fn get_val(main: Option<&Value>, cap: &str, attr: &str) -> Option<Value> {
            main.and_then(|m| m.get(cap))
                .and_then(|c| c.get(attr))
                .and_then(|a| a.get("value"))
                .cloned()
        }

        let mut data = serde_json::Map::new();

        if let Some(v) = get_val(main, "presenceSensor", "presence") {
            data.insert("presence".into(), v);
        }
        if let Some(v) = get_val(main, "motionSensor", "motion") {
            data.insert("motion".into(), v);
        }

        let temp_val = get_val(main, "temperatureMeasurement", "temperature");
        if temp_val.is_some() {
            let unit = main
                .and_then(|m| m.get("temperatureMeasurement"))
                .and_then(|t| t.get("temperature"))
                .and_then(|t| t.get("unit"))
                .cloned();
            data.insert(
                "temperature".into(),
                serde_json::json!({ "value": temp_val, "unit": unit }),
            );
        }

        if let Some(v) = get_val(main, "relativeHumidityMeasurement", "humidity") {
            data.insert("humidity".into(), v);
        }
        if let Some(v) = get_val(main, "illuminanceMeasurement", "illuminance") {
            data.insert("illuminance".into(), v);
        }
        if let Some(v) = get_val(main, "contactSensor", "contact") {
            data.insert("contact".into(), v);
        }

        let sound = get_val(main, "soundDetection", "soundDetected")
            .or_else(|| get_val(main, "soundDetection", "soundDetectionState"));
        if let Some(v) = sound {
            data.insert("sound_detection".into(), v);
        }

        if let Some(v) = get_val(main, "battery", "battery") {
            data.insert("battery".into(), v);
        }

        ToolResult::ok(Value::Object(data))
    }
}
