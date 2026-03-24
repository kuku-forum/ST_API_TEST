from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "scenes"
LABEL = "🎬 씬 (Scenes)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/scenes")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("sceneId", items[0]["sceneId"])
    return result


def _execute(ctx: TestContext) -> ApiCallResult:
    sid = ctx.store.get("sceneId")
    if not sid:
        raise ValueError("sceneId 없음 — 씬이 등록되어 있지 않음")
    return ctx.client.request("POST", f"/scenes/{sid}/execute")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY, LABEL, "씬 목록 조회", "GET /scenes — 모든 씬 목록", _list
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "씬 실행",
        "POST /scenes/{id}/execute — 첫 번째 씬 실행",
        _execute,
        has_side_effect=True,
    ),
]
