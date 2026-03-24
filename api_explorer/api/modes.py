from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "modes"
LABEL = "🔄 모드 (Modes)"


def _list(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음")
    result = ctx.client.request("GET", f"/locations/{lid}/modes")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("modeId", items[0]["id"])
    return result


def _current(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음")
    return ctx.client.request("GET", f"/locations/{lid}/modes/current")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "모드 목록 조회",
        "GET /locations/{id}/modes — 위치의 모드 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "현재 모드 조회",
        "GET /locations/{id}/modes/current — 현재 활성 모드",
        _current,
    ),
]
