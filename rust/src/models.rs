use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::client::ApiClient;

#[derive(Debug, Clone, Serialize)]
pub struct RequestInfo {
    pub method: String,
    pub url: String,
    pub body: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseInfo {
    pub status: i32,
    pub status_text: String,
    pub body: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiCallResult {
    pub request: RequestInfo,
    pub response: ResponseInfo,
    #[serde(rename = "durationMs")]
    pub duration_ms: u128,
}

pub type EndpointFn = fn(&mut TestContext) -> Result<Vec<ApiCallResult>, String>;

#[derive(Clone)]
pub struct ApiEndpointTest {
    pub category: String,
    pub category_label: String,
    pub name: String,
    pub description: String,
    pub test: EndpointFn,
    pub has_side_effect: bool,
    pub needs_setup: Option<String>,
}

impl ApiEndpointTest {
    pub fn new(
        category: &str,
        category_label: &str,
        name: &str,
        description: &str,
        test: EndpointFn,
    ) -> Self {
        Self {
            category: category.to_string(),
            category_label: category_label.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            test,
            has_side_effect: false,
            needs_setup: None,
        }
    }

    pub fn with_side_effect(mut self) -> Self {
        self.has_side_effect = true;
        self
    }

    pub fn with_needs_setup(mut self, needs_setup: &str) -> Self {
        self.needs_setup = Some(needs_setup.to_string());
        self
    }
}

pub struct TestContext {
    pub client: ApiClient,
    pub store: HashMap<String, String>,
    pub run_side_effects: bool,
}

impl TestContext {
    pub fn set_store(&mut self, key: &str, value: impl Into<String>) {
        self.store.insert(key.to_string(), value.into());
    }

    pub fn get_store(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    pub fn has_store(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
}

#[derive(Clone)]
pub struct EndpointTestResult {
    pub endpoint: ApiEndpointTest,
    pub calls: Vec<ApiCallResult>,
    pub success: bool,
    pub skipped: bool,
    pub error: Option<String>,
}

pub struct TestSuiteResult {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration_ms: u128,
    pub results: Vec<EndpointTestResult>,
}
