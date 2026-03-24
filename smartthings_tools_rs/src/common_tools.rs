use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::schemas::common::*;
use crate::tool::{Tool, ToolResult};

fn extract_error(response: &std::collections::HashMap<String, Value>) -> Option<String> {
    response.get("error").map(|e| {
        if let Some(obj) = e.as_object() {
            obj.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or(&e.to_string())
                .to_string()
        } else {
            e.to_string()
        }
    })
}

pub fn extract_error_msg(response: &std::collections::HashMap<String, Value>) -> String {
    extract_error(response).unwrap_or_else(|| "Unknown error".to_string())
}

pub fn has_error(response: &std::collections::HashMap<String, Value>) -> bool {
    response.contains_key("error")
}

pub struct ListDevicesTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for ListDevicesTool<'_> {
    fn name(&self) -> &'static str {
        "list_devices"
    }

    fn description(&self) -> &'static str {
        "등록된 모든 SmartThings 디바이스 목록을 조회합니다. 각 디바이스의 ID, 이름, 방, 지원 capability를 반환합니다."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ListDevicesInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: ListDevicesInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let mut params = std::collections::HashMap::new();
        if let Some(loc) = &input.location_id {
            params.insert("locationId".to_string(), loc.clone());
        }
        if let Some(cap) = &input.capability {
            params.insert("capability".to_string(), cap.clone());
        }

        let p = if params.is_empty() {
            None
        } else {
            Some(params)
        };
        let response = self.client.get("/devices", p);

        if let Some(err) = extract_error(&response) {
            return ToolResult::fail_with_data(
                err,
                serde_json::to_value(&response).unwrap_or_default(),
            );
        }

        let items = response
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let devices: Vec<Value> = items
            .iter()
            .map(|item| {
                let components = item.get("components").and_then(|c| c.as_array());
                let first = components.and_then(|c| c.first());
                let caps: Vec<Value> = first
                    .and_then(|f| f.get("capabilities"))
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|cap| cap.get("id").cloned())
                            .collect()
                    })
                    .unwrap_or_default();

                serde_json::json!({
                    "device_id": item.get("deviceId"),
                    "label": item.get("label"),
                    "room_id": item.get("roomId"),
                    "capabilities": caps,
                })
            })
            .collect();

        let count = devices.len();
        ToolResult::ok(serde_json::json!({ "devices": devices, "count": count }))
    }
}

pub struct GetDeviceStatusTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GetDeviceStatusTool<'_> {
    fn name(&self) -> &'static str {
        "get_device_status"
    }

    fn description(&self) -> &'static str {
        "특정 디바이스의 현재 상태를 조회합니다. 전원, 온도, 밝기 등 모든 capability 상태를 반환합니다."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GetDeviceStatusInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GetDeviceStatusInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self
            .client
            .get(&format!("/devices/{}/status", input.device_id), None);

        if let Some(err) = extract_error(&response) {
            return ToolResult::fail_with_data(
                err,
                serde_json::to_value(&response).unwrap_or_default(),
            );
        }

        let main = response
            .get("components")
            .and_then(|c| c.get("main"))
            .and_then(|m| m.as_object());

        let mut flattened = serde_json::Map::new();
        if let Some(main_obj) = main {
            for (capability, attributes) in main_obj {
                if let Some(attr_obj) = attributes.as_object() {
                    for (attribute, payload) in attr_obj {
                        if let Some(value) = payload.get("value") {
                            flattened.insert(format!("{capability}.{attribute}"), value.clone());
                        }
                    }
                }
            }
        }

        ToolResult::ok(serde_json::json!({
            "device_id": input.device_id,
            "status": flattened,
        }))
    }
}

pub struct SendCommandTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SendCommandTool<'_> {
    fn name(&self) -> &'static str {
        "send_command"
    }

    fn description(&self) -> &'static str {
        "범용 디바이스 커맨드를 전송합니다. 다른 도구로 해결되지 않는 특수한 capability/command 조합에 사용합니다."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SendCommandInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SendCommandInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self.client.command(
            &input.device_id,
            &input.capability,
            &input.command,
            Some(input.arguments),
            Some(&input.component),
        );

        if let Some(err) = extract_error(&response) {
            return ToolResult::fail_with_data(
                err,
                serde_json::to_value(&response).unwrap_or_default(),
            );
        }

        ToolResult::ok(serde_json::json!({
            "device_id": input.device_id,
            "result": response,
        }))
    }
}

pub struct ExecuteSceneTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for ExecuteSceneTool<'_> {
    fn name(&self) -> &'static str {
        "execute_scene"
    }

    fn description(&self) -> &'static str {
        "SmartThings에 등록된 씬을 실행합니다. 여러 디바이스를 한 번에 제어할 수 있습니다."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ExecuteSceneInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: ExecuteSceneInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self.client.post(
            &format!("/scenes/{}/execute", input.scene_id),
            Some(serde_json::json!({})),
        );

        if let Some(err) = extract_error(&response) {
            return ToolResult::fail_with_data(
                err,
                serde_json::to_value(&response).unwrap_or_default(),
            );
        }

        ToolResult::ok(serde_json::json!({
            "scene_id": input.scene_id,
            "result": response,
        }))
    }
}

pub struct GetWeatherTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for GetWeatherTool<'_> {
    fn name(&self) -> &'static str {
        "get_weather"
    }

    fn description(&self) -> &'static str {
        "현재 위치의 날씨 정보(기온, 습도, 날씨 상태)를 조회합니다."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(GetWeatherInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: GetWeatherInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let response = self.client.get(
            &format!(
                "/services/coordinate/locations/{}/weather",
                input.location_id
            ),
            None,
        );

        if let Some(err) = extract_error(&response) {
            return ToolResult::fail_with_data(
                err,
                serde_json::to_value(&response).unwrap_or_default(),
            );
        }

        ToolResult::ok(serde_json::to_value(&response).unwrap_or_default())
    }
}
