use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RobotVacuumInput {
    pub device_id: String,
    pub action: String,
    pub cleaning_mode: Option<String>,
    pub turbo: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DoorLockInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WasherInput {
    pub device_id: String,
    pub action: String,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DryerInput {
    pub device_id: String,
    pub action: String,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DishwasherInput {
    pub device_id: String,
    pub action: String,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RefrigeratorInput {
    pub device_id: String,
    pub feature: String,
    pub state: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ThermostatInput {
    pub device_id: String,
    pub mode: Option<String>,
    pub heating_setpoint: Option<f64>,
    pub cooling_setpoint: Option<f64>,
    pub fan_mode: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AlarmInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SecuritySystemInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GarageDoorInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValveInput {
    pub device_id: String,
    pub action: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SafetySensorInput {
    pub device_id: String,
}
