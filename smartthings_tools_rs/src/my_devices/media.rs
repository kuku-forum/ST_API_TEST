use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct MediaPlaybackTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for MediaPlaybackTool<'_> {
    fn name(&self) -> &'static str {
        "media_playback"
    }
    fn description(&self) -> &'static str {
        "미디어 재생을 제어합니다. 재생, 일시정지, 정지, 빨리감기, 되감기를 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(MediaPlaybackInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: MediaPlaybackInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data =
            self.client
                .command(&input.device_id, "mediaPlayback", &input.action, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "action": input.action }))
    }
}

pub struct MediaVolumeTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for MediaVolumeTool<'_> {
    fn name(&self) -> &'static str {
        "media_volume"
    }
    fn description(&self) -> &'static str {
        "미디어 디바이스(스피커 등)의 볼륨을 조절합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(MediaVolumeInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: MediaVolumeInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = match input.action.as_str() {
            "set" => {
                let vol = match input.volume {
                    Some(v) => v,
                    None => return ToolResult::fail("volume is required when action is 'set'."),
                };
                self.client.command(
                    &input.device_id,
                    "audioVolume",
                    "setVolume",
                    Some(vec![Value::from(vol)]),
                    None,
                )
            }
            "up" => self
                .client
                .command(&input.device_id, "audioVolume", "volumeUp", None, None),
            "down" => {
                self.client
                    .command(&input.device_id, "audioVolume", "volumeDown", None, None)
            }
            other => return ToolResult::fail(format!("Unsupported volume action: {other}")),
        };
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "volume": input.volume }),
        )
    }
}
