from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "rooms"
LABEL = "🚪 방 (Rooms)"


def _list(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음")
    result = ctx.client.request("GET", f"/locations/{lid}/rooms")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("roomId", items[0]["roomId"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    rid = ctx.store.get("roomId")
    if not lid or not rid:
        raise ValueError("locationId 또는 roomId 없음")
    return ctx.client.request("GET", f"/locations/{lid}/rooms/{rid}")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "방 목록 조회",
        "GET /locations/{id}/rooms — 위치 내 방 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 방 상세 조회",
        "GET /locations/{id}/rooms/{roomId} — 방 상세 정보",
        _get,
    ),
]
