use serde_json::json;

use crate::models::{ApiCallResult, ApiEndpointTest, TestContext};

const CAT_DEVICES: &str = "devices";
const CAT_DEVICE_PROFILES: &str = "deviceProfiles";
const CAT_CAPABILITIES: &str = "capabilities";

const LABEL_DEVICES: &str = "📱 디바이스";
const LABEL_DEVICE_PROFILES: &str = "🔧 디바이스 프로필";
const LABEL_CAPABILITIES: &str = "⚡ 기능";

pub fn tests() -> Vec<ApiEndpointTest> {
    vec![
        ApiEndpointTest::new(
            CAT_DEVICES,
            LABEL_DEVICES,
            "디바이스 목록 조회",
            "GET /devices — 모든 디바이스 목록",
            devices_list,
        ),
        ApiEndpointTest::new(
            CAT_DEVICES,
            LABEL_DEVICES,
            "특정 디바이스 상세 조회",
            "GET /devices/{id} — 디바이스 상세 정보",
            devices_get,
        ),
        ApiEndpointTest::new(
            CAT_DEVICES,
            LABEL_DEVICES,
            "디바이스 전체 상태 조회",
            "GET /devices/{id}/status — 모든 컴포넌트/기능 상태",
            devices_status,
        ),
        ApiEndpointTest::new(
            CAT_DEVICES,
            LABEL_DEVICES,
            "디바이스 연결 상태 조회",
            "GET /devices/{id}/health — ONLINE/OFFLINE 상태",
            devices_health,
        ),
        ApiEndpointTest::new(
            CAT_DEVICES,
            LABEL_DEVICES,
            "디바이스 명령 실행 (refresh)",
            "POST /devices/{id}/commands — refresh 명령 (안전)",
            devices_refresh,
        )
        .with_side_effect(),
        ApiEndpointTest::new(
            CAT_DEVICE_PROFILES,
            LABEL_DEVICE_PROFILES,
            "디바이스 프로필 목록 조회",
            "GET /deviceprofiles — 등록된 프로필 목록",
            device_profiles_list,
        ),
        ApiEndpointTest::new(
            CAT_DEVICE_PROFILES,
            LABEL_DEVICE_PROFILES,
            "특정 디바이스 프로필 조회",
            "GET /deviceprofiles/{id} — 프로필 상세 정보",
            device_profiles_get,
        )
        .with_needs_setup("등록된 디바이스 프로필이 필요합니다"),
        ApiEndpointTest::new(
            CAT_CAPABILITIES,
            LABEL_CAPABILITIES,
            "기능 네임스페이스 목록 조회",
            "GET /capabilities/namespaces — 기능 네임스페이스 목록",
            capabilities_namespaces,
        ),
        ApiEndpointTest::new(
            CAT_CAPABILITIES,
            LABEL_CAPABILITIES,
            "switch 기능 상세 조회",
            "GET /capabilities/switch/1 — 표준 switch 기능 정의",
            capabilities_switch,
        ),
        ApiEndpointTest::new(
            CAT_CAPABILITIES,
            LABEL_CAPABILITIES,
            "디바이스 기반 기능 조회",
            "첫 번째 디바이스의 첫 기능 상세 조회",
            capabilities_device_cap,
        ),
    ]
}

fn devices_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/devices", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(device_id) = first.get("deviceId").and_then(|v| v.as_str()) {
                    ctx.set_store("deviceId", device_id);
                }

                let caps = first
                    .get("components")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|main| main.get("capabilities"))
                    .and_then(|v| v.as_array())
                    .map(|caps_arr| {
                        caps_arr
                            .iter()
                            .filter_map(|cap| cap.get("id").and_then(|v| v.as_str()))
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .unwrap_or_default();

                ctx.set_store("deviceCapabilities", caps);
            }
        }
    }
    Ok(vec![result])
}

fn devices_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let did = ctx
        .get_store("deviceId")
        .ok_or_else(|| "deviceId 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/devices/{did}"), None, None)?;
    Ok(vec![result])
}

fn devices_status(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let did = ctx
        .get_store("deviceId")
        .ok_or_else(|| "deviceId 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/devices/{did}/status"), None, None)?;
    Ok(vec![result])
}

fn devices_health(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let did = ctx
        .get_store("deviceId")
        .ok_or_else(|| "deviceId 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/devices/{did}/health"), None, None)?;
    Ok(vec![result])
}

fn devices_refresh(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let did = ctx
        .get_store("deviceId")
        .ok_or_else(|| "deviceId 없음".to_string())?;
    let payload = json!({
        "commands": [
            {
                "component": "main",
                "capability": "refresh",
                "command": "refresh"
            }
        ]
    });
    let result = ctx.client.request(
        "POST",
        &format!("/devices/{did}/commands"),
        Some(payload),
        None,
    )?;
    Ok(vec![result])
}

fn device_profiles_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/deviceprofiles", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(id) = first.get("id").and_then(|v| v.as_str()) {
                    ctx.set_store("deviceProfileId", id);
                }
            }
        }
    }
    Ok(vec![result])
}

fn device_profiles_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let profile_id = ctx
        .get_store("deviceProfileId")
        .ok_or_else(|| "등록된 프로필 없음 — 목록이 비어있습니다".to_string())?;
    let result = ctx.client.request(
        "GET",
        &format!("/deviceprofiles/{profile_id}"),
        None,
        None,
    )?;
    Ok(vec![result])
}

fn capabilities_namespaces(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx
        .client
        .request("GET", "/capabilities/namespaces", None, None)?;
    Ok(vec![result])
}

fn capabilities_switch(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx
        .client
        .request("GET", "/capabilities/switch/1", None, None)?;
    Ok(vec![result])
}

fn capabilities_device_cap(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let cap_id = ctx
        .get_store("deviceCapabilities")
        .and_then(|caps| {
            caps.split(',')
                .map(str::trim)
                .find(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "temperatureMeasurement".to_string());

    let result = ctx
        .client
        .request("GET", &format!("/capabilities/{cap_id}/1"), None, None)?;
    Ok(vec![result])
}
