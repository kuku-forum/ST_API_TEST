pub mod client;
pub mod common_tools;
pub mod extended_devices;
pub mod my_devices;
pub mod schemas;
pub mod tool;

use client::SmartThingsClient;
use serde_json::Value;
use tool::{Tool, ToolResult};

pub struct SmartThingsToolkit {
    client: SmartThingsClient,
    include_extended: bool,
}

impl SmartThingsToolkit {
    pub fn new(token: impl Into<String>, include_extended: bool) -> Self {
        Self {
            client: SmartThingsClient::new(token),
            include_extended,
        }
    }

    pub fn get_tools(&self) -> Vec<Box<dyn Tool + '_>> {
        let mut tools: Vec<Box<dyn Tool + '_>> = Vec::new();

        tools.push(Box::new(common_tools::ListDevicesTool {
            client: &self.client,
        }));
        tools.push(Box::new(common_tools::GetDeviceStatusTool {
            client: &self.client,
        }));
        tools.push(Box::new(common_tools::SendCommandTool {
            client: &self.client,
        }));
        tools.push(Box::new(common_tools::ExecuteSceneTool {
            client: &self.client,
        }));
        tools.push(Box::new(common_tools::GetWeatherTool {
            client: &self.client,
        }));

        tools.extend(my_devices::get_my_device_tools(&self.client));

        if self.include_extended {
            tools.extend(extended_devices::get_extended_device_tools(&self.client));
        }

        tools
    }

    pub fn list_tool_names(&self) -> Vec<&'static str> {
        self.get_tools().iter().map(|t| t.name()).collect()
    }

    pub fn execute(&self, tool_name: &str, args: Value) -> ToolResult {
        let tools = self.get_tools();
        match tools.iter().find(|t| t.name() == tool_name) {
            Some(tool) => tool.execute(args),
            None => ToolResult::fail(format!("Unknown tool: {tool_name}")),
        }
    }

    pub fn to_openai_tools(&self) -> Vec<Value> {
        self.get_tools()
            .iter()
            .map(|t| t.to_openai_tool())
            .collect()
    }

    pub fn to_anthropic_tools(&self) -> Vec<Value> {
        self.get_tools()
            .iter()
            .map(|t| t.to_anthropic_tool())
            .collect()
    }

    pub fn to_function_schemas(&self) -> Vec<Value> {
        self.get_tools()
            .iter()
            .map(|t| t.to_function_schema())
            .collect()
    }
}
