use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

use crate::models::{ApiCallResult, RequestInfo, ResponseInfo};

const BASE_URL: &str = "https://api.smartthings.com/v1";

#[derive(Clone)]
pub struct ApiClient {
    token: String,
    agent: ureq::Agent,
}

impl ApiClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            agent: ureq::AgentBuilder::new().build(),
        }
    }

    pub fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        query: Option<HashMap<String, String>>,
    ) -> Result<ApiCallResult, String> {
        let url = format!("{BASE_URL}{path}");
        let mut req = self
            .agent
            .request(method, &url)
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Accept", "application/json");

        if let Some(query_map) = query {
            for (k, v) in &query_map {
                req = req.query(k, v);
            }
        }

        let request_body = body.clone();
        let started = Instant::now();

        let response_result = if let Some(payload) = body {
            req.send_json(payload)
        } else {
            req.call()
        };

        let duration_ms = started.elapsed().as_millis();

        match response_result {
            Ok(resp) => Ok(Self::to_call_result(resp, method, &url, request_body, duration_ms)),
            Err(ureq::Error::Status(_, resp)) => {
                Ok(Self::to_call_result(resp, method, &url, request_body, duration_ms))
            }
            Err(ureq::Error::Transport(err)) => Err(format!("네트워크 오류: {}", err)),
        }
    }

    fn to_call_result(
        resp: ureq::Response,
        method: &str,
        fallback_url: &str,
        request_body: Option<Value>,
        duration_ms: u128,
    ) -> ApiCallResult {
        let status = i32::from(resp.status());
        let status_text = resp.status_text().to_string();
        let final_url = resp.get_url().to_string();

        let response_text = resp.into_string().ok();
        let response_body = response_text.as_ref().and_then(|txt| {
            if txt.trim().is_empty() {
                None
            } else {
                serde_json::from_str::<Value>(txt)
                    .ok()
                    .or_else(|| Some(Value::String(txt.clone())))
            }
        });

        ApiCallResult {
            request: RequestInfo {
                method: method.to_string(),
                url: if final_url.is_empty() {
                    fallback_url.to_string()
                } else {
                    final_url
                },
                body: request_body,
            },
            response: ResponseInfo {
                status,
                status_text,
                body: response_body,
            },
            duration_ms,
        }
    }
}
