use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::RefrigeratorInput;
use crate::tool::{Tool, ToolResult};

pub struct RefrigeratorTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for RefrigeratorTool<'_> {
    fn name(&self) -> &'static str {
        "refrigerator_control"
    }
    fn description(&self) -> &'static str {
        "냉장고의 급속냉각, 급속냉동, 제상 기능을 제어합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(RefrigeratorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: RefrigeratorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let cmd = match input.feature.as_str() {
            "rapidCooling" => "setRapidCooling",
            "rapidFreezing" => "setRapidFreezing",
            "defrost" => "setDefrost",
            other => return ToolResult::fail(format!("Unsupported feature: {other}")),
        };

        let data = self.client.command(
            &input.device_id,
            "refrigeration",
            cmd,
            Some(vec![Value::String(input.state.clone())]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::to_value(&data).unwrap_or_default())
    }
}
