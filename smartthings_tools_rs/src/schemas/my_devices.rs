use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SwitchPowerInput {
    pub device_id: String,
    pub state: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetBrightnessInput {
    pub device_id: String,
    pub level: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetColorInput {
    pub device_id: String,
    pub hue: i32,
    pub saturation: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetColorTemperatureInput {
    pub device_id: String,
    pub temperature: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlCurtainInput {
    pub device_id: String,
    pub action: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TvPowerInput {
    pub device_id: String,
    pub state: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TvVolumeInput {
    pub device_id: String,
    pub action: String,
    pub volume: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TvMuteInput {
    pub device_id: String,
    pub state: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TvChannelInput {
    pub device_id: String,
    pub action: String,
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TvInputInput {
    pub device_id: String,
    pub source: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MediaPlaybackInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MediaVolumeInput {
    pub device_id: String,
    pub action: String,
    pub volume: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AcControlInput {
    pub device_id: String,
    pub power: Option<String>,
    pub mode: Option<String>,
    pub temperature: Option<f64>,
    pub wind_level: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AirPurifierInput {
    pub device_id: String,
    pub power: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DehumidifierInput {
    pub device_id: String,
    pub power: Option<String>,
    pub mode: Option<String>,
    pub target_humidity: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSensorDataInput {
    pub device_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetEnergyDataInput {
    pub device_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetBatteryInput {
    pub device_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OvenStatusInput {
    pub device_id: String,
}
