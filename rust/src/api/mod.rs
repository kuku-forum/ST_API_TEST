pub mod apps;
pub mod automations;
pub mod devices;
pub mod locations;
pub mod misc;

use crate::models::ApiEndpointTest;

pub fn all_tests() -> Vec<ApiEndpointTest> {
    let mut tests = Vec::new();
    tests.extend(locations::tests());
    tests.extend(devices::tests());
    tests.extend(automations::tests());
    tests.extend(apps::tests());
    tests.extend(misc::tests());
    tests
}

pub fn categories(tests: &[ApiEndpointTest]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in tests {
        if !out.contains(&t.category) {
            out.push(t.category.clone());
        }
    }
    out
}

pub fn get_tests_by_category(tests: &[ApiEndpointTest], category: &str) -> Vec<ApiEndpointTest> {
    tests
        .iter()
        .filter(|t| t.category == category)
        .cloned()
        .collect()
}
