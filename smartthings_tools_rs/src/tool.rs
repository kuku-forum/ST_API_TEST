use schemars::schema::RootSchema;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct ToolResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResult {
    pub fn ok(data: Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn fail(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }

    pub fn fail_with_data(error: impl Into<String>, data: Value) -> Self {
        Self {
            success: false,
            data: Some(data),
            error: Some(error.into()),
        }
    }
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> RootSchema;
    fn execute(&self, args: Value) -> ToolResult;

    fn to_openai_tool(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": self.description(),
                "parameters": self.parameters_schema(),
            }
        })
    }

    fn to_anthropic_tool(&self) -> Value {
        serde_json::json!({
            "name": self.name(),
            "description": self.description(),
            "input_schema": self.parameters_schema(),
        })
    }

    fn to_function_schema(&self) -> Value {
        serde_json::json!({
            "name": self.name(),
            "description": self.description(),
            "parameters": self.parameters_schema(),
        })
    }
}
