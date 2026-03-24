use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::schemas::my_devices::SwitchPowerInput;
use crate::tool::{Tool, ToolResult};

pub struct SwitchPowerTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SwitchPowerTool<'_> {
    fn name(&self) -> &'static str {
        "switch_power"
    }
    fn description(&self) -> &'static str {
        "디바이스 전원을 켜거나 끕니다. 조명, 스마트플러그, 공기청정기, 제습기, TV, 환풍기 등 switch capability가 있는 모든 디바이스에 사용합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SwitchPowerInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SwitchPowerInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "switch", &input.state, None, None);
        if data.contains_key("error") {
            let msg = crate::common_tools::extract_error_msg(&data);
            return ToolResult::fail(msg);
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "state": input.state }))
    }
}
