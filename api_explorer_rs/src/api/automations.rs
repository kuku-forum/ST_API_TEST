use std::collections::HashMap;

use crate::models::{ApiCallResult, ApiEndpointTest, TestContext};

const CAT_SCENES: &str = "scenes";
const CAT_RULES: &str = "rules";

const LABEL_SCENES: &str = "🎬 씬";
const LABEL_RULES: &str = "📐 규칙";

pub fn tests() -> Vec<ApiEndpointTest> {
    vec![
        ApiEndpointTest::new(
            CAT_SCENES,
            LABEL_SCENES,
            "씬 목록 조회",
            "GET /scenes — 모든 씬 목록",
            scenes_list,
        ),
        ApiEndpointTest::new(
            CAT_SCENES,
            LABEL_SCENES,
            "씬 실행",
            "POST /scenes/{id}/execute — 첫 번째 씬 실행",
            scenes_execute,
        )
        .with_side_effect(),
        ApiEndpointTest::new(
            CAT_RULES,
            LABEL_RULES,
            "규칙 목록 조회",
            "GET /rules?locationId={id} — 위치의 자동화 규칙 목록",
            rules_list,
        ),
        ApiEndpointTest::new(
            CAT_RULES,
            LABEL_RULES,
            "특정 규칙 상세 조회",
            "GET /rules/{id} — 규칙 상세 정보",
            rules_get,
        )
        .with_needs_setup("등록된 규칙이 필요합니다"),
    ]
}

fn scenes_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/scenes", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(scene_id) = first.get("sceneId").and_then(|v| v.as_str()) {
                    ctx.set_store("sceneId", scene_id);
                }
            }
        }
    }
    Ok(vec![result])
}

fn scenes_execute(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let scene_id = ctx
        .get_store("sceneId")
        .ok_or_else(|| "sceneId 없음 — 씬이 등록되어 있지 않음".to_string())?;
    let result = ctx
        .client
        .request("POST", &format!("/scenes/{scene_id}/execute"), None, None)?;
    Ok(vec![result])
}

fn rules_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음".to_string())?;
    let mut query = HashMap::new();
    query.insert("locationId".to_string(), location_id);

    let result = ctx.client.request("GET", "/rules", None, Some(query))?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(rule_id) = first.get("id").and_then(|v| v.as_str()) {
                    ctx.set_store("ruleId", rule_id);
                }
            }
        }
    }

    Ok(vec![result])
}

fn rules_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let rule_id = ctx
        .get_store("ruleId")
        .ok_or_else(|| "ruleId 없음 — 등록된 규칙 없음".to_string())?;
    let query = ctx.get_store("locationId").map(|location_id| {
        let mut q = HashMap::new();
        q.insert("locationId".to_string(), location_id);
        q
    });

    let result = ctx
        .client
        .request("GET", &format!("/rules/{rule_id}"), None, query)?;
    Ok(vec![result])
}
