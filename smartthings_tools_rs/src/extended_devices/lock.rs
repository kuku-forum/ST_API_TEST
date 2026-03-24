use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::DoorLockInput;
use crate::tool::{Tool, ToolResult};

pub struct DoorLockTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for DoorLockTool<'_> {
    fn name(&self) -> &'static str {
        "door_lock_control"
    }
    fn description(&self) -> &'static str {
        "스마트 도어록을 잠그거나 해제합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(DoorLockInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: DoorLockInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "lock", &input.action, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::to_value(&data).unwrap_or_default())
    }
}
