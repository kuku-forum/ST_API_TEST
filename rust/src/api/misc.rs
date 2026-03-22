use std::collections::HashMap;

use crate::models::{ApiCallResult, ApiEndpointTest, TestContext};

const CAT_SCHEMA: &str = "schema";
const CAT_SERVICES: &str = "services";
const CAT_HISTORY: &str = "history";

const LABEL_SCHEMA: &str = "🔌 스키마 커넥터";
const LABEL_SERVICES: &str = "🌤️ 서비스";
const LABEL_HISTORY: &str = "📜 이벤트 기록";

pub fn tests() -> Vec<ApiEndpointTest> {
    vec![
        ApiEndpointTest::new(
            CAT_SCHEMA,
            LABEL_SCHEMA,
            "스키마 앱 목록 조회",
            "GET /schema/apps — ST Schema C2C 커넥터 목록",
            schema_list,
        ),
        ApiEndpointTest::new(
            CAT_SERVICES,
            LABEL_SERVICES,
            "날씨 정보 조회",
            "GET /services/coordinate/locations/{id}/weather — 현재 날씨",
            services_weather,
        ),
        ApiEndpointTest::new(
            CAT_HISTORY,
            LABEL_HISTORY,
            "디바이스 이벤트 기록 조회",
            "GET /history/devices — 최근 디바이스 이벤트",
            history_devices,
        ),
    ]
}

fn schema_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/schema/apps", None, None)?;
    Ok(vec![result])
}

fn services_weather(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음".to_string())?;

    let first = ctx.client.request(
        "GET",
        &format!("/services/coordinate/locations/{location_id}/weather"),
        None,
        None,
    )?;

    if first.response.status == 404 {
        let alt = ctx
            .client
            .request("GET", &format!("/locations/{location_id}/weather"), None, None)?;
        return Ok(vec![first, alt]);
    }

    Ok(vec![first])
}

fn history_devices(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let device_id = ctx
        .get_store("deviceId")
        .ok_or_else(|| "deviceId 또는 locationId 없음".to_string())?;
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "deviceId 또는 locationId 없음".to_string())?;

    let mut query = HashMap::new();
    query.insert("locationId".to_string(), location_id);
    query.insert("deviceId".to_string(), device_id);

    let result = ctx
        .client
        .request("GET", "/history/devices", None, Some(query))?;
    Ok(vec![result])
}
