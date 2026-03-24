use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct GetEnergyDataTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GetEnergyDataTool<'_> {
    fn name(&self) -> &'static str {
        "get_energy_data"
    }
    fn description(&self) -> &'static str {
        "스마트 플러그의 전력 사용량을 조회합니다. 현재 전력(W)과 누적 에너지(kWh)를 반환합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GetEnergyDataInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GetEnergyDataInput = match serde_json::from_value(args) {
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

        let power = main
            .and_then(|m| m.get("powerMeter"))
            .and_then(|p| p.get("power"))
            .and_then(|p| p.get("value"));

        let energy = main
            .and_then(|m| m.get("energyMeter"))
            .and_then(|e| e.get("energy"))
            .and_then(|e| e.get("value"));

        ToolResult::ok(serde_json::json!({ "power_w": power, "energy_kwh": energy }))
    }
}

pub struct GetBatteryStatusTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GetBatteryStatusTool<'_> {
    fn name(&self) -> &'static str {
        "get_battery_status"
    }
    fn description(&self) -> &'static str {
        "배터리 구동 디바이스의 배터리 잔량을 확인합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GetBatteryInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GetBatteryInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);
        if has_error(&response) {
            return ToolResult::fail(extract_error_msg(&response));
        }

        let battery = response
            .get("components")
            .and_then(|c| c.get("main"))
            .and_then(|m| m.get("battery"))
            .and_then(|b| b.get("battery"))
            .and_then(|b| b.get("value"));

        ToolResult::ok(serde_json::json!({ "battery_percent": battery }))
    }
}
