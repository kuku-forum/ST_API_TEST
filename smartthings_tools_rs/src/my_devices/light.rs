use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct SetBrightnessTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SetBrightnessTool<'_> {
    fn name(&self) -> &'static str {
        "set_brightness"
    }
    fn description(&self) -> &'static str {
        "조명 밝기를 0~100 사이 값으로 설정합니다. 라이트스트립, 디머 조명 등에 사용합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SetBrightnessInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SetBrightnessInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "switchLevel",
            "setLevel",
            Some(vec![Value::from(input.level)]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "level": input.level }))
    }
}

pub struct SetColorTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SetColorTool<'_> {
    fn name(&self) -> &'static str {
        "set_color"
    }
    fn description(&self) -> &'static str {
        "조명 색상을 변경합니다. hue(색조 0~100)와 saturation(채도 0~100)을 설정합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SetColorInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SetColorInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "colorControl",
            "setColor",
            Some(vec![
                serde_json::json!({"hue": input.hue, "saturation": input.saturation}),
            ]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({
            "device_id": input.device_id,
            "hue": input.hue,
            "saturation": input.saturation,
        }))
    }
}

pub struct SetColorTemperatureTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for SetColorTemperatureTool<'_> {
    fn name(&self) -> &'static str {
        "set_color_temperature"
    }
    fn description(&self) -> &'static str {
        "조명 색온도를 변경합니다. 2700K(따뜻한 노란색)~6500K(차가운 흰색) 범위를 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(SetColorTemperatureInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: SetColorTemperatureInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "colorTemperature",
            "setColorTemperature",
            Some(vec![Value::from(input.temperature)]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "temperature": input.temperature }),
        )
    }
}
