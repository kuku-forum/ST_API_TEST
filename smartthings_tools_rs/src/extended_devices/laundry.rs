use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct WasherTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for WasherTool<'_> {
    fn name(&self) -> &'static str {
        "washer_control"
    }
    fn description(&self) -> &'static str {
        "세탁기를 제어합니다. 가동, 일시정지, 정지 및 세탁 모드를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(WasherInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: WasherInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "washerOperatingState",
            "setMachineState",
            Some(vec![Value::String(input.action.clone())]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        if let Some(ref mode) = input.mode {
            let data = self.client.command(
                &input.device_id,
                "washerMode",
                "setWasherMode",
                Some(vec![Value::String(mode.clone())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
        }

        ToolResult::ok(serde_json::to_value(&data).unwrap_or_default())
    }
}

pub struct DryerTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for DryerTool<'_> {
    fn name(&self) -> &'static str {
        "dryer_control"
    }
    fn description(&self) -> &'static str {
        "건조기를 제어합니다. 가동, 일시정지, 정지 및 건조 모드를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(DryerInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: DryerInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "dryerOperatingState",
            "setMachineState",
            Some(vec![Value::String(input.action.clone())]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }

        if let Some(ref mode) = input.mode {
            let data = self.client.command(
                &input.device_id,
                "dryerMode",
                "setDryerMode",
                Some(vec![Value::String(mode.clone())]),
                None,
            );
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
        }

        ToolResult::ok(serde_json::to_value(&data).unwrap_or_default())
    }
}
