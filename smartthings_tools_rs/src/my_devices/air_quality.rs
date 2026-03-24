use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct AirPurifierTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for AirPurifierTool<'_> {
    fn name(&self) -> &'static str {
        "air_purifier_control"
    }
    fn description(&self) -> &'static str {
        "공기청정기를 제어합니다. 전원과 운전 모드를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(AirPurifierInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: AirPurifierInput = match serde_json::from_value(args) {
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
                "airPurifierFanMode",
                "setAirPurifierFanMode",
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

        ToolResult::ok(Value::Object(results))
    }
}

pub struct DehumidifierTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for DehumidifierTool<'_> {
    fn name(&self) -> &'static str {
        "dehumidifier_control"
    }
    fn description(&self) -> &'static str {
        "제습기를 제어합니다. 전원, 모드, 목표 습도를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(DehumidifierInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: DehumidifierInput = match serde_json::from_value(args) {
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
                "samsungce.dehumidifierMode",
                "setDehumidifierMode",
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

        if let Some(humidity) = input.target_humidity {
            let data = self.client.command(
                &input.device_id,
                "samsungce.relativeHumidityLevel",
                "setHumidityLevel",
                Some(vec![Value::from(humidity)]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            results.insert(
                "target_humidity".to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        ToolResult::ok(Value::Object(results))
    }
}
