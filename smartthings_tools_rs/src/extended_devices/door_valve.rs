use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct GarageDoorTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GarageDoorTool<'_> {
    fn name(&self) -> &'static str {
        "garage_door_control"
    }
    fn description(&self) -> &'static str {
        "차고문을 열거나 닫습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GarageDoorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GarageDoorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "doorControl", &input.action, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "result": data }),
        )
    }
}

pub struct ValveTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for ValveTool<'_> {
    fn name(&self) -> &'static str {
        "valve_control"
    }
    fn description(&self) -> &'static str {
        "스마트 밸브(수도 차단 밸브 등)를 열거나 닫습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ValveInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: ValveInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "valve", &input.action, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "result": data }),
        )
    }
}
