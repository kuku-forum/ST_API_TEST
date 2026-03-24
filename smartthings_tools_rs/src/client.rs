use serde_json::Value;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.smartthings.com/v1";

pub struct SmartThingsClient {
    token: String,
    agent: ureq::Agent,
}

impl SmartThingsClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            agent: ureq::AgentBuilder::new().build(),
        }
    }

    pub fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        params: Option<HashMap<String, String>>,
    ) -> HashMap<String, Value> {
        let url = format!("{BASE_URL}{path}");
        let mut req = self
            .agent
            .request(method, &url)
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Accept", "application/json")
            .set("Content-Type", "application/json");

        if let Some(ref query) = params {
            for (k, v) in query {
                req = req.query(k, v);
            }
        }

        let resp_result = if let Some(payload) = body {
            req.send_json(payload)
        } else {
            req.call()
        };

        match resp_result {
            Ok(resp) => Self::parse_response(resp),
            Err(ureq::Error::Status(code, resp)) => {
                let mut data = Self::parse_response(resp);
                data.insert("status_code".to_string(), Value::Number(code.into()));
                data
            }
            Err(ureq::Error::Transport(err)) => {
                let mut m = HashMap::new();
                m.insert("error".to_string(), Value::String(err.to_string()));
                m.insert("status_code".to_string(), Value::Number(0.into()));
                m
            }
        }
    }

    fn parse_response(resp: ureq::Response) -> HashMap<String, Value> {
        let text = resp.into_string().unwrap_or_default();
        if text.trim().is_empty() {
            return HashMap::new();
        }
        match serde_json::from_str::<HashMap<String, Value>>(&text) {
            Ok(map) => map,
            Err(_) => {
                let mut m = HashMap::new();
                m.insert("raw".to_string(), Value::String(text));
                m
            }
        }
    }

    pub fn get(
        &self,
        path: &str,
        params: Option<HashMap<String, String>>,
    ) -> HashMap<String, Value> {
        self.request("GET", path, None, params)
    }

    pub fn post(&self, path: &str, body: Option<Value>) -> HashMap<String, Value> {
        self.request("POST", path, body, None)
    }

    pub fn command(
        &self,
        device_id: &str,
        capability: &str,
        command: &str,
        arguments: Option<Vec<Value>>,
        component: Option<&str>,
    ) -> HashMap<String, Value> {
        let comp = component.unwrap_or("main");
        let args = arguments.unwrap_or_default();
        let body = serde_json::json!({
            "commands": [{
                "component": comp,
                "capability": capability,
                "command": command,
                "arguments": args,
            }]
        });
        self.post(&format!("/devices/{device_id}/commands"), Some(body))
    }
}
