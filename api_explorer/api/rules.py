from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "rules"
LABEL = "📐 규칙 (Rules)"


def _list(ctx: TestContext) -> ApiCallResult:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음")
    result = ctx.client.request("GET", "/rules", query={"locationId": lid})
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("ruleId", items[0]["id"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    rid = ctx.store.get("ruleId")
    lid = ctx.store.get("locationId")
    if not rid:
        raise ValueError("ruleId 없음 — 등록된 규칙 없음")
    q = {"locationId": lid} if lid else None
    return ctx.client.request("GET", f"/rules/{rid}", query=q)


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "규칙 목록 조회",
        "GET /rules?locationId={id} — 위치의 자동화 규칙 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 규칙 상세 조회",
        "GET /rules/{id} — 규칙 상세 정보",
        _get,
        needs_setup="등록된 규칙이 필요합니다",
    ),
]
