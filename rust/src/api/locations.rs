use crate::models::{ApiCallResult, ApiEndpointTest, TestContext};

const CAT_LOCATIONS: &str = "locations";
const CAT_ROOMS: &str = "rooms";
const CAT_MODES: &str = "modes";

const LABEL_LOCATIONS: &str = "📍 위치";
const LABEL_ROOMS: &str = "🚪 방";
const LABEL_MODES: &str = "🔄 모드";

pub fn tests() -> Vec<ApiEndpointTest> {
    vec![
        ApiEndpointTest::new(
            CAT_LOCATIONS,
            LABEL_LOCATIONS,
            "위치 목록 조회",
            "GET /locations — 모든 SmartThings 위치 목록",
            locations_list,
        ),
        ApiEndpointTest::new(
            CAT_LOCATIONS,
            LABEL_LOCATIONS,
            "특정 위치 상세 조회",
            "GET /locations/{id} — 위치 상세 정보",
            locations_get,
        ),
        ApiEndpointTest::new(
            CAT_ROOMS,
            LABEL_ROOMS,
            "방 목록 조회",
            "GET /locations/{id}/rooms — 위치 내 방 목록",
            rooms_list,
        ),
        ApiEndpointTest::new(
            CAT_ROOMS,
            LABEL_ROOMS,
            "특정 방 상세 조회",
            "GET /locations/{id}/rooms/{roomId} — 방 상세 정보",
            rooms_get,
        ),
        ApiEndpointTest::new(
            CAT_MODES,
            LABEL_MODES,
            "모드 목록 조회",
            "GET /locations/{id}/modes — 위치의 모드 목록",
            modes_list,
        ),
        ApiEndpointTest::new(
            CAT_MODES,
            LABEL_MODES,
            "현재 모드 조회",
            "GET /locations/{id}/modes/current — 현재 활성 모드",
            modes_current,
        ),
    ]
}

fn locations_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/locations", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(location_id) = first.get("locationId").and_then(|v| v.as_str()) {
                    ctx.set_store("locationId", location_id);
                }
            }
        }
    }
    Ok(vec![result])
}

fn locations_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음 — 위치 목록 조회 먼저 실행 필요".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/locations/{location_id}"), None, None)?;
    Ok(vec![result])
}

fn rooms_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/locations/{location_id}/rooms"), None, None)?;

    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(room_id) = first.get("roomId").and_then(|v| v.as_str()) {
                    ctx.set_store("roomId", room_id);
                }
            }
        }
    }

    Ok(vec![result])
}

fn rooms_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 또는 roomId 없음".to_string())?;
    let room_id = ctx
        .get_store("roomId")
        .ok_or_else(|| "locationId 또는 roomId 없음".to_string())?;

    let result = ctx.client.request(
        "GET",
        &format!("/locations/{location_id}/rooms/{room_id}"),
        None,
        None,
    )?;
    Ok(vec![result])
}

fn modes_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/locations/{location_id}/modes"), None, None)?;

    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(mode_id) = first.get("id").and_then(|v| v.as_str()) {
                    ctx.set_store("modeId", mode_id);
                }
            }
        }
    }

    Ok(vec![result])
}

fn modes_current(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let location_id = ctx
        .get_store("locationId")
        .ok_or_else(|| "locationId 없음".to_string())?;
    let result = ctx.client.request(
        "GET",
        &format!("/locations/{location_id}/modes/current"),
        None,
        None,
    )?;
    Ok(vec![result])
}
