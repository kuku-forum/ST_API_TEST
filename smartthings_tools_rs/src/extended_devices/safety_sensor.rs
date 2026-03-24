use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::SafetySensorInput;
use crate::tool::{Tool, ToolResult};

pub struct SmokeDetectorTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SmokeDetectorTool<'_> {
    fn name(&self) -> &'static str {
        "smoke_detector_status"
    }
    fn description(&self) -> &'static str {
        "화재 감지기의 현재 상태를 조회합니다. (읽기 전용)"
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SafetySensorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SafetySensorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        let smoke = data
            .get("components")
            .and_then(|c| c.get("main"))
            .and_then(|m| m.get("smokeDetector"))
            .and_then(|s| s.get("smoke"))
            .and_then(|s| s.get("value"));

        ToolResult::ok(serde_json::json!({ "smoke": smoke, "device_id": input.device_id }))
    }
}

pub struct CoDetectorTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for CoDetectorTool<'_> {
    fn name(&self) -> &'static str {
        "co_detector_status"
    }
    fn description(&self) -> &'static str {
        "일산화탄소 감지기의 현재 상태를 조회합니다. (읽기 전용)"
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SafetySensorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SafetySensorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        let co = data
            .get("components")
            .and_then(|c| c.get("main"))
            .and_then(|m| m.get("carbonMonoxideDetector"))
            .and_then(|c| c.get("carbonMonoxide"))
            .and_then(|c| c.get("value"));

        ToolResult::ok(serde_json::json!({ "carbon_monoxide": co, "device_id": input.device_id }))
    }
}

pub struct WaterLeakTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for WaterLeakTool<'_> {
    fn name(&self) -> &'static str {
        "water_leak_status"
    }
    fn description(&self) -> &'static str {
        "누수 감지 센서의 현재 상태를 조회합니다. (읽기 전용)"
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SafetySensorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SafetySensorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        let water = data
            .get("components")
            .and_then(|c| c.get("main"))
            .and_then(|m| m.get("waterSensor"))
            .and_then(|w| w.get("water"))
            .and_then(|w| w.get("value"));

        ToolResult::ok(serde_json::json!({ "water": water, "device_id": input.device_id }))
    }
}
