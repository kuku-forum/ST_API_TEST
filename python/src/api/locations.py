from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "locations"
LABEL = "📍 위치 (Locations)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/locations")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("locationId", items[0]["locationId"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음 — 위치 목록 조회 먼저 실행 필요")
    return ctx.client.request("GET", f"/locations/{lid}")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "위치 목록 조회",
        "GET /locations — 모든 SmartThings 위치 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 위치 상세 조회",
        "GET /locations/{id} — 위치 상세 정보",
        _get,
    ),
]
