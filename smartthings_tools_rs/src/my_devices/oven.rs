use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::has_error;
use crate::schemas::my_devices::OvenStatusInput;
use crate::tool::{Tool, ToolResult};

pub struct OvenStatusTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for OvenStatusTool<'_> {
    fn name(&self) -> &'static str {
        "oven_status"
    }
    fn description(&self) -> &'static str {
        "오븐/쿠커의 현재 상태를 조회합니다. 운전 모드, 동작 상태, 온도 등을 확인할 수 있습니다. (읽기 전용)"
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(OvenStatusInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: OvenStatusInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&response) {
            return ToolResult::fail(crate::common_tools::extract_error_msg(&response));
        }

        let main = response.get("components").and_then(|c| c.get("main"));

        let oven_mode = main
            .and_then(|m| m.get("ovenMode"))
            .and_then(|o| o.get("ovenMode"))
            .and_then(|o| o.get("value"));

        let operating_state = main
            .and_then(|m| m.get("ovenOperatingState"))
            .and_then(|o| o.get("machineState"))
            .and_then(|o| o.get("value"));

        let temp_value = main
            .and_then(|m| m.get("temperatureMeasurement"))
            .and_then(|t| t.get("temperature"))
            .and_then(|t| t.get("value"));

        let temp_unit = main
            .and_then(|m| m.get("temperatureMeasurement"))
            .and_then(|t| t.get("temperature"))
            .and_then(|t| t.get("unit"));

        ToolResult::ok(serde_json::json!({
            "oven_mode": oven_mode,
            "oven_operating_state": operating_state,
            "temperature": { "value": temp_value, "unit": temp_unit },
        }))
    }
}
