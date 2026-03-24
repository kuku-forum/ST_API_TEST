use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::my_devices::*;
use crate::tool::{Tool, ToolResult};

pub struct TvPowerTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for TvPowerTool<'_> {
    fn name(&self) -> &'static str {
        "tv_power"
    }
    fn description(&self) -> &'static str {
        "TV 전원을 켜거나 끕니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(TvPowerInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: TvPowerInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "switch", &input.state, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "state": input.state }))
    }
}

pub struct TvVolumeTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for TvVolumeTool<'_> {
    fn name(&self) -> &'static str {
        "tv_volume"
    }
    fn description(&self) -> &'static str {
        "TV 볼륨을 조절합니다. 직접 설정, 올리기, 내리기를 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(TvVolumeInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: TvVolumeInput = match serde_json::from_value(args) {
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

pub struct TvMuteTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for TvMuteTool<'_> {
    fn name(&self) -> &'static str {
        "tv_mute"
    }
    fn description(&self) -> &'static str {
        "TV 음소거를 설정하거나 해제합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(TvMuteInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: TvMuteInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self
            .client
            .command(&input.device_id, "audioMute", &input.state, None, None);
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "state": input.state }))
    }
}

pub struct TvChannelTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for TvChannelTool<'_> {
    fn name(&self) -> &'static str {
        "tv_channel"
    }
    fn description(&self) -> &'static str {
        "TV 채널을 변경합니다. 채널 번호 직접 입력, 다음/이전 채널 이동을 지원합니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(TvChannelInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: TvChannelInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = match input.action.as_str() {
            "set" => {
                let ch = match &input.channel {
                    Some(c) => c.clone(),
                    None => return ToolResult::fail("channel is required when action is 'set'."),
                };
                self.client.command(
                    &input.device_id,
                    "tvChannel",
                    "setTvChannel",
                    Some(vec![Value::String(ch)]),
                    None,
                )
            }
            "up" => self
                .client
                .command(&input.device_id, "tvChannel", "channelUp", None, None),
            "down" => self
                .client
                .command(&input.device_id, "tvChannel", "channelDown", None, None),
            other => return ToolResult::fail(format!("Unsupported channel action: {other}")),
        };
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(
            serde_json::json!({ "device_id": input.device_id, "action": input.action, "channel": input.channel }),
        )
    }
}

pub struct TvInputTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for TvInputTool<'_> {
    fn name(&self) -> &'static str {
        "tv_input"
    }
    fn description(&self) -> &'static str {
        "TV 입력 소스를 변경합니다 (HDMI1, HDMI2, USB, digitalTv 등)."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(TvInputInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: TvInputInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };
        let data = self.client.command(
            &input.device_id,
            "mediaInputSource",
            "setInputSource",
            Some(vec![Value::String(input.source.clone())]),
            None,
        );
        if has_error(&data) {
            return ToolResult::fail(extract_error_msg(&data));
        }
        ToolResult::ok(serde_json::json!({ "device_id": input.device_id, "source": input.source }))
    }
}
