pub mod air_quality;
pub mod climate;
pub mod curtain;
pub mod energy;
pub mod light;
pub mod media;
pub mod oven;
pub mod sensor;
pub mod switch;
pub mod tv;

use crate::client::SmartThingsClient;
use crate::tool::Tool;

pub fn get_my_device_tools(client: &SmartThingsClient) -> Vec<Box<dyn Tool + '_>> {
    vec![
        Box::new(switch::SwitchPowerTool { client }),
        Box::new(light::SetBrightnessTool { client }),
        Box::new(light::SetColorTool { client }),
        Box::new(light::SetColorTemperatureTool { client }),
        Box::new(curtain::ControlCurtainTool { client }),
        Box::new(tv::TvPowerTool { client }),
        Box::new(tv::TvVolumeTool { client }),
        Box::new(tv::TvMuteTool { client }),
        Box::new(tv::TvChannelTool { client }),
        Box::new(tv::TvInputTool { client }),
        Box::new(media::MediaPlaybackTool { client }),
        Box::new(media::MediaVolumeTool { client }),
        Box::new(climate::AcControlTool { client }),
        Box::new(air_quality::AirPurifierTool { client }),
        Box::new(air_quality::DehumidifierTool { client }),
        Box::new(oven::OvenStatusTool { client }),
        Box::new(sensor::GetSensorDataTool { client }),
        Box::new(energy::GetEnergyDataTool { client }),
        Box::new(energy::GetBatteryStatusTool { client }),
    ]
}
