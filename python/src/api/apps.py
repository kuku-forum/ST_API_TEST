from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "apps"
LABEL = "📦 앱 (Apps)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/apps")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("appId", items[0]["appId"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    aid = ctx.store.get("appId")
    if not aid:
        raise ValueError("appId 없음 — 등록된 앱 없음")
    return ctx.client.request("GET", f"/apps/{aid}")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY, LABEL, "앱 목록 조회", "GET /apps — 등록된 SmartApp 목록", _list
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 앱 상세 조회",
        "GET /apps/{id} — 앱 상세 정보",
        _get,
        needs_setup="등록된 앱이 필요합니다",
    ),
]
