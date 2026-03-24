use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::Value;

use crate::client::SmartThingsClient;
use crate::common_tools::{extract_error_msg, has_error};
use crate::schemas::extended_devices::ThermostatInput;
use crate::tool::{Tool, ToolResult};

pub struct ThermostatTool<'a> {
    pub client: &'a SmartThingsClient,
}

impl Tool for ThermostatTool<'_> {
    fn name(&self) -> &'static str {
        "thermostat_control"
    }
    fn description(&self) -> &'static str {
        "온도조절기/보일러를 제어합니다. 운전 모드, 난방 목표 온도, 냉방 목표 온도, 팬 모드를 설정할 수 있습니다."
    }
    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ThermostatInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        let input: ThermostatInput = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
        };

        let mut commands: Vec<(&str, &str, &str, Vec<Value>)> = Vec::new();

        if let Some(ref mode) = input.mode {
            commands.push((
                "mode",
                "thermostatMode",
                "setThermostatMode",
                vec![Value::String(mode.clone())],
            ));
        }
        if let Some(heat) = input.heating_setpoint {
            commands.push((
                "heating_setpoint",
                "thermostatHeatingSetpoint",
                "setHeatingSetpoint",
                vec![serde_json::json!(heat)],
            ));
        }
        if let Some(cool) = input.cooling_setpoint {
            commands.push((
                "cooling_setpoint",
                "thermostatCoolingSetpoint",
                "setCoolingSetpoint",
                vec![serde_json::json!(cool)],
            ));
        }
        if let Some(ref fan) = input.fan_mode {
            commands.push((
                "fan_mode",
                "thermostatFanMode",
                "setThermostatFanMode",
                vec![Value::String(fan.clone())],
            ));
        }

        if commands.is_empty() {
            return ToolResult::fail("설정할 항목이 없습니다.");
        }

        let mut applied = serde_json::Map::new();
        let mut responses = serde_json::Map::new();

        for (key, capability, command, arguments) in commands {
            let data =
                self.client
                    .command(&input.device_id, capability, command, Some(arguments), None);
            if has_error(&data) {
                return ToolResult::fail(extract_error_msg(&data));
            }
            applied.insert(key.to_string(), serde_json::json!(key));
            responses.insert(
                key.to_string(),
                serde_json::to_value(&data).unwrap_or_default(),
            );
        }

        ToolResult::ok(serde_json::json!({
            "device_id": input.device_id,
            "set": applied,
            "results": responses,
        }))
    }
}
