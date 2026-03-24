use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::RobotVacuumInput;
use crate::tool::{Tool, ToolResult};

pub struct RobotVacuumTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for RobotVacuumTool<'_> {
    fn name(&self) -> &'static str {
        "robot_vacuum_control"
    }
    fn description(&self) -> &'static str {
        "로봇청소기를 제어합니다. 청소 시작/일시정지/복귀, 청소 모드, 터보 모드를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(RobotVacuumInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: RobotVacuumInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let movement = match input.action.as_str() {
            "start" => "cleaning",
            "pause" => "pause",
            "returnToHome" => "homing",
            other => return ToolResult::fail(format!("Unsupported action: {other}")),
        };

        let data = self.client.command(
            &input.device_id,
            "robotCleanerMovement",
            "setRobotCleanerMovement",
            Some(vec![Value::String(movement.to_string())]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        if let Some(ref mode) = input.cleaning_mode {
            let data = self.client.command(
                &input.device_id,
                "robotCleanerCleaningMode",
                "setRobotCleanerCleaningMode",
                Some(vec![Value::String(mode.clone())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
        }

        if let Some(ref turbo) = input.turbo {
            let data = self.client.command(
                &input.device_id,
                "robotCleanerTurboMode",
                "setRobotCleanerTurboMode",
                Some(vec![Value::String(turbo.clone())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
        }

        ToolResult::ok(serde_json::to_value(&data).unwrap_or_default())
    }
}
