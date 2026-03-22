use crate::models::{ApiCallResult, ApiEndpointTest, TestContext};

const CAT_APPS: &str = "apps";
const CAT_INSTALLED_APPS: &str = "installedApps";
const CAT_SUBSCRIPTIONS: &str = "subscriptions";
const CAT_SCHEDULES: &str = "schedules";

const LABEL_APPS: &str = "📦 앱";
const LABEL_INSTALLED_APPS: &str = "📲 설치된 앱";
const LABEL_SUBSCRIPTIONS: &str = "🔔 구독";
const LABEL_SCHEDULES: &str = "⏰ 스케줄";

pub fn tests() -> Vec<ApiEndpointTest> {
    vec![
        ApiEndpointTest::new(
            CAT_APPS,
            LABEL_APPS,
            "앱 목록 조회",
            "GET /apps — 등록된 SmartApp 목록",
            apps_list,
        ),
        ApiEndpointTest::new(
            CAT_APPS,
            LABEL_APPS,
            "특정 앱 상세 조회",
            "GET /apps/{id} — 앱 상세 정보",
            apps_get,
        )
        .with_needs_setup("등록된 앱이 필요합니다"),
        ApiEndpointTest::new(
            CAT_INSTALLED_APPS,
            LABEL_INSTALLED_APPS,
            "설치된 앱 목록 조회",
            "GET /installedapps — 설치된 앱 목록",
            installed_apps_list,
        ),
        ApiEndpointTest::new(
            CAT_INSTALLED_APPS,
            LABEL_INSTALLED_APPS,
            "특정 설치된 앱 상세 조회",
            "GET /installedapps/{id} — 설치된 앱 상세 정보",
            installed_apps_get,
        ),
        ApiEndpointTest::new(
            CAT_SUBSCRIPTIONS,
            LABEL_SUBSCRIPTIONS,
            "구독 목록 조회",
            "GET /installedapps/{id}/subscriptions — 이벤트 구독 목록",
            subscriptions_list,
        )
        .with_needs_setup("설치된 앱(installedApp)이 필요합니다"),
        ApiEndpointTest::new(
            CAT_SCHEDULES,
            LABEL_SCHEDULES,
            "스케줄 목록 조회",
            "GET /installedapps/{id}/schedules — 예약 실행 목록",
            schedules_list,
        )
        .with_needs_setup("설치된 앱(installedApp)이 필요합니다"),
    ]
}

fn apps_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/apps", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(app_id) = first.get("appId").and_then(|v| v.as_str()) {
                    ctx.set_store("appId", app_id);
                }
            }
        }
    }
    Ok(vec![result])
}

fn apps_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let app_id = ctx
        .get_store("appId")
        .ok_or_else(|| "appId 없음 — 등록된 앱 없음".to_string())?;
    let result = ctx
        .client
        .request("GET", &format!("/apps/{app_id}"), None, None)?;
    Ok(vec![result])
}

fn installed_apps_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let result = ctx.client.request("GET", "/installedapps", None, None)?;
    if result.response.status == 200 {
        if let Some(items) = result
            .response
            .body
            .as_ref()
            .and_then(|b| b.get("items"))
            .and_then(|v| v.as_array())
        {
            if let Some(first) = items.first() {
                if let Some(id) = first.get("installedAppId").and_then(|v| v.as_str()) {
                    ctx.set_store("installedAppId", id);
                }
            }
        }
    }
    Ok(vec![result])
}

fn installed_apps_get(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let installed_app_id = ctx
        .get_store("installedAppId")
        .ok_or_else(|| "installedAppId 없음".to_string())?;
    let result = ctx.client.request(
        "GET",
        &format!("/installedapps/{installed_app_id}"),
        None,
        None,
    )?;
    Ok(vec![result])
}

fn subscriptions_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let installed_app_id = ctx
        .get_store("installedAppId")
        .ok_or_else(|| "installedAppId 없음 — 설치된 앱 필요".to_string())?;
    let result = ctx.client.request(
        "GET",
        &format!("/installedapps/{installed_app_id}/subscriptions"),
        None,
        None,
    )?;
    Ok(vec![result])
}

fn schedules_list(ctx: &mut TestContext) -> Result<Vec<ApiCallResult>, String> {
    let installed_app_id = ctx
        .get_store("installedAppId")
        .ok_or_else(|| "installedAppId 없음 — 설치된 앱 필요".to_string())?;
    let result = ctx.client.request(
        "GET",
        &format!("/installedapps/{installed_app_id}/schedules"),
        None,
        None,
    )?;
    Ok(vec![result])
}
