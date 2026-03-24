use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::ControlCurtainInput;
use crate::tool::{Tool, ToolResult};

pub struct ControlCurtainTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for ControlCurtainTool<'_> {
    fn name(&self) -> &'static str {
        "control_curtain"
    }
    fn description(&self) -> &'static str {
        "커튼/블라인드를 열거나 닫습니다. 특정 위치(0~100%)로 설정할 수도 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ControlCurtainInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: ControlCurtainInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let mut shade_result = serde_json::json!(null);
        let mut level_result = serde_json::json!(null);

        if let Some(ref action) = input.action {
            if !["open", "close", "pause"].contains(&action.as_str()) {
                return ToolResult::fail(format!("Unsupported curtain action: {action}"));
            }
            let data = self
                .client
                .command(&input.device_id, "windowShade", action, None, None);
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            shade_result = serde_json::to_value(&data).unwrap_or_default();
        }

        if let Some(level) = input.level {
            let data = self.client.command(
                &input.device_id,
                "switchLevel",
                "setLevel",
                Some(vec![Value::from(level)]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            level_result = serde_json::to_value(&data).unwrap_or_default();
        }

        if input.action.is_none() && input.level.is_none() {
            return ToolResult::fail("Either action or level must be provided.");
        }

        ToolResult::ok(serde_json::json!({
            "device_id": input.device_id,
            "action": input.action,
            "level": input.level,
            "shade_result": shade_result,
            "level_result": level_result,
        }))
    }
}
