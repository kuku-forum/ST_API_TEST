pub mod dishwasher;
pub mod door_valve;
pub mod laundry;
pub mod lock;
pub mod refrigerator;
pub mod safety_sensor;
pub mod security;
pub mod thermostat;
pub mod vacuum;

use crate::client::SmartThingsClient;
use crate::tool::Tool;

pub fn get_extended_device_tools(client: &SmartThingsClient) -> Vec<Box<dyn Tool + '_>> {
    vec![
        Box::new(vacuum::RobotVacuumTool { client }),
        Box::new(lock::DoorLockTool { client }),
        Box::new(laundry::WasherTool { client }),
        Box::new(laundry::DryerTool { client }),
        Box::new(dishwasher::DishwasherTool { client }),
        Box::new(refrigerator::RefrigeratorTool { client }),
        Box::new(thermostat::ThermostatTool { client }),
        Box::new(security::AlarmTool { client }),
        Box::new(security::SecuritySystemTool { client }),
        Box::new(door_valve::GarageDoorTool { client }),
        Box::new(door_valve::ValveTool { client }),
        Box::new(safety_sensor::SmokeDetectorTool { client }),
        Box::new(safety_sensor::CoDetectorTool { client }),
        Box::new(safety_sensor::WaterLeakTool { client }),
    ]
}
