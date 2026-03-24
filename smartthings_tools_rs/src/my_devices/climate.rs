use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::AcControlInput;
use crate::tool::{Tool, ToolResult};

pub struct AcControlTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for AcControlTool<'_> {
    fn name(&self) -> &'static str {
        "ac_control"
    }
    fn description(&self) -> &'static str {
        "에어컨을 제어합니다. 전원, 모드, 온도, 풍량을 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(AcControlInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: AcControlInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let mut results = serde_json::Map::new();

        if let Some(ref power) = input.power {
            let data = self
                .client
                .command(&input.device_id, "switch", power, None, None);
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            results.insert(
                "power".to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        if let Some(ref mode) = input.mode {
            let data = self.client.command(
                &input.device_id,
                "airConditionerMode",
                "setAirConditionerMode",
                Some(vec![Value::String(mode.clone())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            results.insert(
                "mode".to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        if let Some(temp) = input.temperature {
            let data = self.client.command(
                &input.device_id,
                "thermostatCoolingSetpoint",
                "setCoolingSetpoint",
                Some(vec![serde_json::json!(temp)]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            results.insert(
                "temperature".to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        if let Some(wind) = input.wind_level {
            let data = self.client.command(
                &input.device_id,
                "airConditionerFanMode",
                "setFanMode",
                Some(vec![Value::String(wind.to_string())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            results.insert(
                "wind_level".to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        ToolResult::ok(Value::Object(results))
    }
}
