use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDevicesInput {
    pub location_id: Option<String>,
    pub capability: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDeviceStatusInput {
    pub device_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SendCommandInput {
    pub device_id: String,
    pub capability: String,
    pub command: String,
    #[serde(default)]
    pub arguments: Vec<serde_json::Value>,
    #[serde(default = "default_component")]
    pub component: String,
}

fn default_component() -> String {
    "main".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteSceneInput {
    pub scene_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWeatherInput {
    pub location_id: String,
}
