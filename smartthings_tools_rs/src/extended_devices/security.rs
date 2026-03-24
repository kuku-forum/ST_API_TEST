use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct AlarmTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for AlarmTool<'_> {
    fn name(&self) -> &'static str {
        "alarm_control"
    }
    fn description(&self) -> &'static str {
        "사이렌/경보 디바이스를 제어합니다. 사이렌, 스트로브, 동시작동, 끄기를 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(AlarmInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: AlarmInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "alarm", &input.action, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "result": data }),
        )
    }
}

pub struct SecuritySystemTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SecuritySystemTool<'_> {
    fn name(&self) -> &'static str {
        "security_system_control"
    }
    fn description(&self) -> &'static str {
        "보안 시스템의 경계 모드를 설정합니다. 외출 경계, 재실 경계, 해제를 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SecuritySystemInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SecuritySystemInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "securitySystem",
            &input.action,
            None,
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "result": data }),
        )
    }
}
